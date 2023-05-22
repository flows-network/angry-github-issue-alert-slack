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

        let comments = if issue.comments > 0 {
            let octocrab = get_octo(&Default);
            let issue = octocrab.issues(owner, repo);
            let mut comment_inner = "".to_string();
            match issue.list_comments(issue_number).send().await {
                Ok(pages) => {
                    for page in pages {
                        let _body = page.body.unwrap_or("".to_string());
                        send_message_to_channel(&slack_workspace, "ch_in", _body.clone());
                        comment_inner.push_str(&_body);
                    }
                }
                Err(_e) => {}
            }

            comment_inner
        } else {
            "".to_string()
        };

        let system = &format!("You are the co-owner of a github repo, you're watching for issues where participants show strong dis-satisfaction with the issue they encountered, please analyze the wording and make judgement based on the whole context.");
        let question = format!("The issue is titled {issue_title}, labeled {labels}, with body text {issue_body}, comments {comments}, based on this context, please judge how angry the issue has caused the affected people to be, please give me one-word absolute answer, answer [YES] if you think they're angry, with greater than 50% confidence, otherwise [NO]");
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
                send_message_to_channel(&slack_workspace, "ch_mid", r.choice.clone());

                if r.choice.to_ascii_lowercase().contains("yes") {
                    let body = format!("This issue is making people angry, please take immediate actions: {issue_title} by {user}\n{issue_url}");

                    send_message_to_channel(&slack_workspace, &slack_channel, body);
                    return;
                }
                return;
            }
            Err(_e) => {}
        }
    }
}
