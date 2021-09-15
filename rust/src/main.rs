use core::str::FromStr;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::{Channel, ReactionType};
use serenity::model::id::ChannelId;
use serenity::model::{channel::Message, gateway::Ready};
use serenity::utils::MessageBuilder;
use std::thread::sleep;
use std::time::Duration;

use std::env;

struct Handler {
    allowed_channel: ChannelId,
    say: Option<String>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        match &self.say {
            Some(_s) => return,
            _ => {}
        }
        let channel = match msg.channel_id.to_channel(&context).await {
            Ok(channel) => channel,
            Err(why) => {
                eprintln!("Error getting channel: {:?}", why);
                return;
            }
        };

        let allowed = self.allowed_channel == msg.channel_id;
        if !allowed & !msg.is_private() {
            return;
        }

        if msg.content == "/ping" {
            let res = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" used the pin command in the ")
                .mention(&msg.channel_id)
                .push(" channel")
                .build();
            if let Err(why) = msg.channel_id.say(&context.http, &res).await {
                eprintln!("Error sending message: {:?}", why);
            }
        }

        let message = msg.content.to_lowercase();
        if message.starts_with("good morning") {
            eprintln!("hello");

            let res = MessageBuilder::new()
                .push_bold_line_safe("beeps")
                .push("ⓖⓞⓞⓓ ⓜⓞⓡⓝⓘⓝⓖ")
                .build();
            if let Err(why) = msg.channel_id.say(&context.http, &res).await {
                eprintln!("Error sending message: {:?}", why);
            }
        } else if message.starts_with("/chore") {
            let res = MessageBuilder::new().push("a chore").build();

            msg.channel_id.broadcast_typing(&context.http).await;
            sleep(Duration::from_millis(2000));

            if let Err(why) = msg.channel_id.say(&context.http, &res).await {
                eprintln!("Error sending message: {:?}", why);
            }
        } else if message.contains("choretle") {
            eprintln!("reacting");
            msg.react(&context.http, ReactionType::from('👀')).await;
        }
    }

    async fn ready(&self, context: Context, ready: Ready) {
        eprintln!("{} is connected", ready.user.name);
        eprintln!("allowed in {}", self.allowed_channel);

        match &self.say {
            Some(s) => {
                self.allowed_channel.broadcast_typing(&context.http).await;
                sleep(Duration::from_millis(2000));
                let res = MessageBuilder::new().push(s).build();

                if let Err(why) = self.allowed_channel.say(&context.http, &res).await {
                    eprintln!("Error sending message: {:?}", why);
                }
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new().configure(|c| c.prefix("~")); // set the bot's prefix to "~"

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let allowed_channel =
        ChannelId::from_str(&env::var("ALLOWED_CHANNEL_ID").expect("allowed_channel_id")).unwrap();

    let args: Vec<String> = env::args().collect();
    let mut say = None;
    if args.len() > 1 {
        if args[1].eq("say") {
            if args[2..].len() > 0 {
                say = Some(args[2..].join(" "));
            }
        }
    }

    let handler = Handler {
        allowed_channel,
        say,
    };

    let mut client = Client::builder(token)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
