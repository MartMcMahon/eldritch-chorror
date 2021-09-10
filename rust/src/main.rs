// extern crate reqwest;
// use oauth2::{http::HeaderMap, http::Method, url::Url, HttpRequest};
// use std::env::var;

// #[tokio::main]
// async fn main() {
//     let discord_api_path = "https://discord.com/api/v9".to_owned();

//     // println!(
//     //     "{} {}",
//     //     discord_api_path,
//     //     var("DISCORD_CHORROR_TOKEN").expect("ok")
//     // );

//     let client_id = var("CLIENT_ID").expect("client id");
//     let client_secret = var("CLIENT_SECRET").expect("client secret");

//     // async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // let req = oauth2::reqwest::async_http_client(HttpRequest {
//     //     url: Url::parse(discord_api_path).unwrap(),
//     //     method: Method::GET,
//     //     headers: HeaderMap::new(),
//     //     body: vec![],
//     // });
//     // reqwest::async_http_client
//     // let res = reqwest::get(discord_api_path)
//     //     .await?
//     //     .json::<HashMap<String, String>>()
//     //     .await?;
//     // println!("{:#?}", res);
//     // Ok(())

//     // let client = reqwest::Client::new();

//     // build url
//     let auth_url = "https://discord.com/api/oauth2/authorize";
//     let scope = "bot";
//     let permissions = "1";

//     // let url = format!(
//     //     "{}?client_id={}&scope={}&permissions={}",
//     //     auth_url, client_id, scope, permissions
//     // );

//     let url = "https://discord.com/api/v9/users/id";
//     let authorization = "Bot ".to_owned() + &client_secret;

//     let client = reqwest::Client::new();
//     let res = client
//         .get(url)
//         .header(reqwest::header::AUTHORIZATION, authorization)
//         .header(
//             reqwest::header::CONTENT_TYPE,
//             "application/x-www-form-urlencoded",
//         )
//         .header(reqwest::header::USER_AGENT, "DiscordBot (url, 0.1)")
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();

//     // let body = reqwest::get(url).await.unwrap().text().await.unwrap();
//     println!("{:?}", res);
// }

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::{channel::Message, gateway::Ready};
use serenity::utils::MessageBuilder;

use std::env;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(why) => {
                    eprintln!("Error getting channel: {:?}", why);
                    return;
                }
            };

            let res = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" used the pin command in the ")
                .mention(&channel)
                .push(" channel")
                .build();
            if let Err(why) = msg.channel_id.say(&context.http, &res).await {
                eprintln!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        eprintln!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
