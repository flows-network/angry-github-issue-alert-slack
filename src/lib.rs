use dotenv::dotenv;
use github_flows::{listen_to_event, EventPayload, GithubLogin::Provided};
use openai_flows::chat::{ChatModel, ChatOptions};
use openai_flows::OpenAIFlows;
use slack_flows::send_message_to_channel;
use std::{env, fmt::format};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    dotenv().ok();
    let github_login = env::var("github_login").unwrap_or("alabulei1".to_string());
    let github_owner = env::var("github_owner").unwrap_or("alabulei1".to_string());
    let github_repo = env::var("github_repo").unwrap_or("a-test".to_string());

    listen_to_event(
        &Provided(github_login),
        &github_owner,
        &github_repo,
        vec!["issues", "issue_comment"],
        handler,
    )
    .await;

    Ok(())
}

async fn handler(payload: EventPayload) {
    // let openai_key_name = env::var("openai_key_name").unwrap_or("secondstate".to_string());
    let slack_workspace = env::var("slack_workspace").unwrap_or("secondstate".to_string());
    let slack_channel = env::var("slack_channel").unwrap_or("github-status".to_string());

    let mut issue = None;

    match payload {
        EventPayload::IssuesEvent(e) => {
            issue = Some(e.issue);
        }

        EventPayload::IssueCommentEvent(e) => {
            issue = Some(e.issue);
        }

        _ => (),
    }

    if let Some(issue) = issue {
        let issue_title = issue.title;
        let issue_number = issue.number;
        let issue_body = issue.body.unwrap();
        let issue_url = issue.html_url;
        let user = issue.user.login;
        let labels = issue.labels;
        let mut openai = OpenAIFlows::new();

        let system = &format!("You are the co-owner of the github repo, you're watching for new issues where the person who raised the issue shows strong dis-satisfaction with the problem he experienced with the project, please analyze the language and situation in the issue.");
        let question = format!("The issue is titled {issue_title}, with body text {issue_body}, please reply by saying YES if the situation is bad, otherwise NO");
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
                if r.choice == "YES" {
                    let body = format!(
                        r#"A new issue that needs your help: {issue_title} by {user} 
                    {issue_url}"#
                    );

                    send_message_to_channel(&slack_workspace, &slack_channel, body);
                    return;
                }
                return;
            }
            Err(_e) => {}
        }
    }
}
