use core::str::FromStr;
use rand::Rng;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::{Channel, ReactionType};
use serenity::model::id::ChannelId;
use serenity::model::{channel::Message, gateway::Ready};
use serenity::utils::MessageBuilder;
use std::env;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

struct Handler {
    allowed_channel: ChannelId,
    say: Option<String>,
}

enum RarityType {
    Spicy,
    Rare,
    Uncommon,
    Common,
}

struct Rarity {
    n: i32,
    rarity_type: RarityType,
    rarity_str: String,
    list: Vec<String>,
}
impl Rarity {
    fn new(roll: i32) -> Self {
        let rarity_type = match roll {
            1..=3 => RarityType::Spicy,
            4..=13 => RarityType::Rare,
            14..=43 => RarityType::Uncommon,
            44..=100 => RarityType::Common,
            _ => RarityType::Common,
        };
        let rarity_str = match roll {
            1..=3 => "spicy".to_owned(),
            4..=13 => "rare".to_owned(),
            14..=43 => "uncommon".to_owned(),
            44..=100 => "common".to_owned(),
            _ => "none".to_owned(),
        };
        let file_string = fs::read_to_string(&rarity_str).expect("error reading file");
        let list: Vec<String> = file_string.split("\n").map(|s| s.to_owned()).collect();
        println!("there are {} items in list", list.len());

        Rarity {
            n: roll,
            rarity_type,
            rarity_str,
            list,
        }
    }

    fn roll_line(&self) -> &str {
        let n = rand::thread_rng().gen_range(0..self.list.len());
        self.list[n].as_str()
    }
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

        let message = msg.content.to_lowercase();
        if message.starts_with("good morning") {
            let res = MessageBuilder::new()
                .push_bold_line_safe("beeps")
                .push("ⓖⓞⓞⓓ ⓜⓞⓡⓝⓘⓝⓖ")
                .build();
            if let Err(why) = msg.channel_id.say(&context.http, &res).await {
                eprintln!("Error sending message: {:?}", why);
            }
        } else if message.starts_with("/chore") {
            let rarity_roll: i32 = rand::thread_rng().gen_range(0..100);
            eprintln!("rarity: {}", rarity_roll);

            let rarity = Rarity::new(rarity_roll);

            let mut line = "";
            while line.is_empty() {
                line = rarity.roll_line();
            }
            let lang = match rarity.rarity_type {
                RarityType::Spicy => Some("diff"),
                RarityType::Rare => Some("md"),
                RarityType::Uncommon => Some("cs"),
                RarityType::Common => None,
            };
            let res = MessageBuilder::new().push_codeblock(line, lang).build();

            msg.channel_id.broadcast_typing(&context.http).await;
            sleep(Duration::from_millis(666));

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
    let rarity_roll = rand::thread_rng().gen_range(1..100);
    let r = Rarity::new(rarity_roll);
    eprintln!("{}", rarity_roll);

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
        } else if args[1].eq("script") {
            say = Some(script());
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

////test long sleeps. say something in a minute
//sleep(Duration::from_millis(60000));
//self.allowed_channel
//    .say(
//        &context.http,
//        MessageBuilder::new().push("it's been a minute").build(),
//    )
//    .await;
//// say in an hour
//sleep(Duration::from_millis(3600000));
//self.allowed_channel
//    .say(
//        &context.http,
//        MessageBuilder::new().push("it's been an hour").build(),
//    )
//    .await;
//sleep(Duration::from_millis(3600000 * 3));
//self.allowed_channel
//    .say(
//        &context.http,
//        MessageBuilder::new().push("it's been 3 hours").build(),
//    )
//    .await;
