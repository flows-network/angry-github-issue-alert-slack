# <p align="center"> Urgent GitHub Issues Alert to Slack </p>

<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/createByTemplate/Urgent-GitHub-Issues-Alert">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[Deploy this function on flows.network](#deploy-your-own-code-review-bot-in-3-simple-steps), and you will get a Slack bot to send you a message when ChatGPT finds an urgent issue from the designed GitHub repo. It helps busy open-source maintainers manage GitHub issues faster! 

<img width="412" alt="image" src="https://github.com/flows-network/urgent-github-issue-alert-slack/assets/45785633/35a6901c-9c6b-448d-aacc-7b0d11ef5c08">

When a new GitHub issue is created, ChatGPT will review the issue title, description, and comments. If ChatGPT thinks this issue is an urgent matter, you will receive a Slack message.

## Deploy your own code review bot in 3 simple steps

1. Create a bot from a template
2. Add your OpenAI API key
3. Configure the bot to review issues on a specified GitHub repo

### 0 Prerequisites

You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

You will also need to sign into [flows.network](https://flows.network/) from your GitHub account. It is free.

### 1 Create a bot from a template

[**Just click here**](https://flows.network/flow/createByTemplate/Summarize-Pull-Request)


Click on the **Create and Build** button.

### 2 Add your OpenAI API key

You will now set up OpenAI integration. Click on **Connect**, enter your key, and give it a name.

[<img width="450" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">](https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png)

Close the tab and go back to the flow.network page once you are done. Click on **Continue**.

### 3 Configure the bot to access GitHub

Next, you will tell the bot which GitHub repo it needs to monitor for upcoming issues to review.

* `github_owner`: GitHub org for the repo *you want to deploy the 🤖 on*.
* `github_repo` : GitHub repo *you want to deploy the 🤖 on*.

> Let's see an example. You would like to deploy the bot to review issues on `WasmEdge/wasmedge_hyper_demo` repo. Here `github_owner = WasmEdge` and `github_repo = wasmedge_hyper_demo`.

Click on the **Connect** or **+ Add new authentication** button to give the function access to the GitHub repo to deploy the 🤖. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to the repo.

[<img width="450" alt="image" src="https://github.com/flows-network/github-pr-summary/assets/45785633/6cefff19-9eeb-4533-a20b-03c6a9c89473">](https://github.com/flows-network/github-pr-summary/assets/45785633/6cefff19-9eeb-4533-a20b-03c6a9c89473)

Close the tab and go back to the flow.network page once you are done. Click on **Deploy**.

### Wait for the magic!

This is it! You are now on the flow details page waiting for the flow function to build. As soon as the flow's status became `running`, the bot is ready to give code reviews! The bot is summoned by every new issue, and every new issue comment.


