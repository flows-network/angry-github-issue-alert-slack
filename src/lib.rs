use dotenv::dotenv;
use flowsnet_platform_sdk::{logger, write_error_log};
use github_flows::{
    get_octo, listen_to_event,
    octocrab::models::events::payload::{IssueCommentEventAction, IssuesEventAction},
    EventPayload,
    GithubLogin::Default,
};
use openai_flows::{
    chat::{ChatModel, ChatOptions},
    OpenAIFlows,
};
use serde::{Deserialize, Serialize};
use slack_flows::send_message_to_channel;
use std::collections::HashSet;
use std::env;
use store_flows::{get, set};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();
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
        let issue_body = issue.body.unwrap_or("".to_string());
        let issue_body = squeeze_fit_comment_texts(&issue_body, "```", 500, 0.6);

        let issue_url = issue.html_url;
        let user = issue.user.login;
        let labels = issue
            .labels
            .into_iter()
            .map(|lab| lab.name)
            .collect::<Vec<String>>()
            .join(", ");

        let mut existing_issues = IssueNumbers::default();

        match get("ISSUE_NUMBERS") {
            Some(issue_numbers_value) => {
                match serde_json::from_value::<IssueNumbers>(issue_numbers_value) {
                    Ok(issue_number_obj) => existing_issues = issue_number_obj,
                    Err(_e) => {
                        write_error_log!("failed to get value from presumed ISSUE_NUMBERS: {_e}");
                    }
                }
            }
            None => {}
        };

        if existing_issues.inner.contains(&issue_number) {
            return;
        }

        let octocrab = get_octo(&Default);
        let issue = octocrab.issues(owner, repo);
        let mut comments = String::new();
        if let Ok(pages) = issue.list_comments(issue_number).send().await {
            for page in pages {
                match page.body {
                    Some(body) => {
                        let comment_body = squeeze_fit_comment_texts(&body, "```", 500, 0.6);
                        comments.push_str(&comment_body);
                    }
                    None => {}
                }
            }
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
            model: ChatModel::GPT35Turbo16K,
            restart: true,
            system_prompt: Some(system),
        };
        match openai.chat_completion(&chat_id, &question, &co).await {
            Ok(r) => {
                if r.choice.to_ascii_lowercase().contains("yes") {
                    existing_issues.inner.insert(issue_number);
                    let data = serde_json::json!(existing_issues);
                    set("ISSUE_NUMBERS", data, None);

                    let body = format!("It appears that this is an urgent matter. Please take immediate action. {issue_title} by {user}\n{issue_url}");
                    send_message_to_channel(&slack_workspace, &slack_channel, body);
                    return;
                }
                return;
            }
            Err(_e) => {
                write_error_log!("openai failed to return result: {_e}");
            }
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
struct IssueNumbers {
    inner: HashSet<u64>,
}

fn squeeze_fit_comment_texts(inp_str: &str, quote_mark: &str, max_len: u16, split: f32) -> String {
    let mut body = String::new();
    let mut inside_quote = false;
    let max_len = max_len as usize;

    for line in inp_str.lines() {
        if line.contains(quote_mark) {
            inside_quote = !inside_quote;
            continue;
        }

        if !inside_quote {
            body.push_str(line);
            body.push('\n');
        }
    }

    let body_len = body.split_whitespace().count();
    let n_take_from_beginning = (max_len as f32 * split) as usize;
    let n_keep_till_end = max_len - n_take_from_beginning;
    match body_len > max_len {
        false => body,
        true => {
            let mut body_text_vec = body.split_whitespace().collect::<Vec<&str>>();
            let drain_to = std::cmp::min(body_len, max_len);
            body_text_vec.drain(n_take_from_beginning..drain_to - n_keep_till_end);
            body_text_vec.join(" ")
        }
    }
}
