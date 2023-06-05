use dotenv::dotenv;
use github_flows::{
    get_octo, listen_to_event,
    octocrab::models::events::payload::{IssueCommentEventAction, IssuesEventAction},
    EventPayload,
    GithubLogin::Default,
};
use openai_flows::chat::{ChatModel, ChatOptions};
use openai_flows::OpenAIFlows;
use slack_flows::send_message_to_channel;
use std::env;
use tiktoken_rs::cl100k_base;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    let github_owner = env::var("github_owner").unwrap_or("alabulei1".to_string());
    let github_repo = env::var("github_repo").unwrap_or("a-test".to_string());

    listen_to_event(
        &Default,
        &github_owner,
        &github_repo,
        vec!["issues", "issue_comment"],
        |payload| handler(&github_owner, &github_repo, payload),
    )
    .await;
}

async fn handler(owner: &str, repo: &str, payload: EventPayload) {
    let slack_workspace = env::var("slack_workspace").unwrap_or("secondstate".to_string());
    let slack_channel = env::var("slack_channel").unwrap_or("github-status".to_string());

    let mut issue = None;

    match payload {
        EventPayload::IssuesEvent(e) => {
            if e.action != IssuesEventAction::Closed {
                issue = Some(e.issue);
            }
        }

        EventPayload::IssueCommentEvent(e) => {
            if e.action != IssueCommentEventAction::Deleted {
                issue = Some(e.issue);
            }
        }

        _ => (),
    }

    if let Some(issue) = issue {
        let issue_title = issue.title;
        let issue_number = issue.number;
        let issue_body = issue.body.unwrap();
        let issue_url = issue.html_url;
        let user = issue.user.login;
        let labels = issue
            .labels
            .into_iter()
            .map(|lab| lab.name)
            .collect::<Vec<String>>()
            .join(", ");

        let mut comments = String::new();

        let octocrab = get_octo(&Default);
        let issue = octocrab.issues(owner, repo);
        if let Ok(pages) = issue.list_comments(issue_number).send().await {
            for page in pages {
                let _body = page.body.unwrap_or("".to_string());
                comments.push_str(&_body);
            }
        }

        let bpe = cl100k_base().unwrap();

        let tokens = bpe.encode_ordinary(&comments);

        if tokens.len() > 2000 {
            let mut token_vec = tokens.to_vec();
            token_vec.truncate(2000);
            comments = match bpe.decode(token_vec) {
                Ok(r) => r,
                Err(_) => comments
                    .split_whitespace()
                    .take(1500)
                    .collect::<Vec<&str>>()
                    .join(" "),
            };
        }
        let system = &format!("You are an AI co-owner of a GitHub repository, monitoring for issues where participants express strong negative sentiment. Your task is to analyze the conversation context based on the issue's title, labels, body text, and comments.");
        let question = format!("An issue titled '{issue_title}', labeled as '{labels}', carries the following body text: '{issue_body}'. The discussion thread includes these comments: '{comments}'. Based on this context, evaluate whether the overall sentiment of this issue is significantly negative. If your confidence in this judgment is greater than 50%, respond in JSON format, a JSON literal only, nothing else:
        {{
            'choice': 'yes or no',
            'confidence': 'confidence'
        }}");
        let chat_id = format!("ISSUE#{issue_number}");

        let mut openai = OpenAIFlows::new();
        openai.set_retry_times(3);

        let co = ChatOptions {
            model: ChatModel::GPT35Turbo,
            restart: true,
            system_prompt: Some(system),
        };
        match openai.chat_completion(&chat_id, &question, &co).await {
            Ok(r) => {
                if r.choice.to_ascii_lowercase().contains("yes") {
                    let body = format!("It appears that this is an urgent matter. Please take immediate action. {issue_title} by {user}\n{issue_url}");

                    send_message_to_channel(&slack_workspace, &slack_channel, body);
                    return;
                }
                return;
            }
            Err(_e) => {}
        }
    }
}
