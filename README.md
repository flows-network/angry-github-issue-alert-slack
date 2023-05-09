# Github Star Slack Messenger

[Deploy this function on flows.network](#deploy-Github-Star-Slack-Messenger-on-your-channel), and you will get a Slack bot that send a message every time your GitHub repo get 10 more stars.

![image](https://user-images.githubusercontent.com/37167103/227172232-e7778f4c-18dc-4eeb-a5f3-6e616bd45e2c.png)

This is a simple example to show how flows.network works when there is a trigger event happens on GitHub then an action gets set off (message sent) in Slack.

## Deploy Github Star Slack Messenger on your Slack channel

To install this Github Star Slack Messenger, we use [flows.network](https://flows.network/), a serverless platform that lets you make a workflow app in three simple steps.

### Fork this repo and make simple code edit

Fork [this repo](https://github.com/flows-network/github-star-slack-messenger) and enter the folder github-star-slack-messenger/src/github-star-slack-messenger.rs/. Replace the parameters in the red boxes below with your GitHub Repo owner and repo name, and then your Slack workspace and channel respectively.

<img width="743" alt="image" src="https://user-images.githubusercontent.com/45785633/227526953-210e366f-9599-4344-b213-d0ff4f185964.png">



### Deploy the code on flows.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Create a Flow" button to start deploying the Github Star Slack Messenger
3. Authorize the [flows.network](https://flows.network/) to access the `slack-calculator` repo you just forked. 

![](https://i.imgur.com/uk1FYW1.png)

4. Click the Deploy button to deploy your function.

### Configure SaaS integrations

Next, flows.network directs you to configure the SaaS integrations required by your flow.

![](https://i.imgur.com/VDhVeLB.png)

Here we need to configue 2 SaaS integrations.

1. Click the "Connect/+ Add new authentication" button to authenticate your **Slack account**. You'll be redirected to a new page to grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on your Slack workspace. The workspace is the parameter you entered at the first step. (Here as can be seen in the screenshot, I already connected the Slack integration)
2.  Click the "Connect/+ Add new authentication" button to authenticate your **GitHub account**. You'll be redirected to a new page to grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on the repo that you changed in the code above.




After that, click the Check button to see your flow details. As soon as the flow function's status turns `ready` and the flow's status becomes `running`, the Github Star Slack Messenger goes live. Get updates right away as your GitHub stars increase!



> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!


## Others


To build locally

```
cargo build target wasm32-wasi --release
```
