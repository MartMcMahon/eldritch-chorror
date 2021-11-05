use chrono::DateTime;
use core::str::FromStr;
use rand::Rng;
use serde::Deserialize;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::StandardFramework;
use serenity::model::id::{MessageId, UserId};
use serenity::model::{
    channel::{Message, ReactionType},
    gateway::Ready,
    id::ChannelId,
};
use serenity::prelude::Mentionable;
use serenity::utils::MessageBuilder;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use std::{
    env,
    fs::{File, OpenOptions},
};

struct Handler {
    allowed_channel: ChannelId,
    mode: Mode,
    args: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum RarityType {
    Spicy,
    Rare,
    Uncommon,
    Common,
}

enum Mode {
    Calc,
    Normal,
    Say,
    Script,
}

struct Roll {
    n: usize,
    line: String,
}

struct Rarity {
    n: i32,
    rarity_type: RarityType,
    rarity_str: String,
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
        let rarity_str = match rarity_type {
            RarityType::Spicy => "spicy".to_owned(),
            RarityType::Rare => "rare".to_owned(),
            RarityType::Uncommon => "uncommon".to_owned(),
            RarityType::Common => "common".to_owned(),
        };

        Rarity {
            n: roll,
            rarity_type,
            rarity_str,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        // HIBERNATING
        let hibernating = false;

        // match &self.mode {
        //     Mode::Normal => {}
        //     _ => return,
        // }
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
            if hibernating {
                return;
            }
            let mut rarity = Rarity::new(100);
            let mut list = vec!["".to_owned()];
            let mut roll = Roll {
                n: 0,
                line: "".to_owned(),
            };

            while roll.line.is_empty() {
                let rarity_roll: i32 = rand::thread_rng().gen_range(0..100);
                rarity = Rarity::new(rarity_roll);
                list = collect_f(rarity.rarity_str.as_str());

                let n = rand::thread_rng().gen_range(0..list.len());

                roll = Roll {
                    n,
                    line: list.remove(n).clone(),
                };
            }
            eprintln!("decided on line {:?}", roll.line);
            eprintln!("is empty: {:?}", roll.line.is_empty());

            // eprintln!("rarity: {:?}", rarity.rarity_type);
            let mut lang = match rarity.rarity_type {
                RarityType::Common => None,
                RarityType::Uncommon => Some("cs"),
                RarityType::Rare => Some("md"),
                RarityType::Spicy => Some("diff"),
            };
            if is_decorated(&roll.line) {
                lang = None;
            }
            let line = match rarity.rarity_type {
                RarityType::Common => roll.line,
                RarityType::Uncommon => "'  ".to_owned() + roll.line.as_str() + "  '",
                RarityType::Rare => "#  ".to_owned() + roll.line.as_str() + "  #",
                RarityType::Spicy => "-  ".to_owned() + roll.line.as_str() + "  -",
            };
            let is_ascend = line.eq("-  Ascend.  -");
            let res = match lang {
                Some(lang) => MessageBuilder::new()
                    .push_codeblock(line, Some(lang))
                    .build(),
                None => MessageBuilder::new().push(line).build(),
            };

            msg.channel_id.broadcast_typing(&context.http).await;
            sleep(Duration::from_millis(666));

            match msg.channel_id.say(&context.http, &res).await {
                Ok(_r) => {
                    if is_ascend {
                        self.allowed_channel.broadcast_typing(&context.http).await;
                        let mut ascended_count = i32::from_str(
                            fs::read_to_string("chores/ascended_count")
                                .expect("error reading ascended_count")
                                .as_str(),
                        )
                        .expect("error converting to i32");
                        ascended_count += 1;
                        fs::write("chores/ascended_count", ascended_count.to_string());
                        if ascended_count >= 7 {
                            if let Err(why) = self
                                .allowed_channel
                                .say(
                                    &context.http,
                                    MessageBuilder::new()
                                        .push("Chores too spicy. Cooldown imminent.".to_owned())
                                        .build(),
                                )
                                .await
                            {
                                eprintln!("Error sending message: {:?}", why);
                            }
                        } else if ascended_count == 10 {
                            // shutdown
                            if let Err(why) = self
                                .allowed_channel
                                .say(
                                    &context.http,
                                    MessageBuilder::new()
                                        .push("Choretle's shell begins to harden!".to_owned())
                                        .build(),
                                )
                                .await
                            {
                                eprintln!("Error sending message: {:?}", why);
                            }
                        } else if ascended_count > 10 {
                            return;
                        }
                    } else {
                        if !is_ascend {
                            increment_count(msg.author.id);
                            remove_line(rarity.rarity_str.as_str(), list)
                        }

                        if rarity.rarity_type == RarityType::Spicy {
                            let user_id = msg.author.id;
                            let scanline = format!("scanning {}...", user_id.mention());

                            let scan_mes = MessageBuilder::new().push(scanline).build();
                            if let Err(why) =
                                self.allowed_channel.say(&context.http, &scan_mes).await
                            {
                                eprintln!("Error sending message: {:?}", why);
                            }

                            self.allowed_channel.broadcast_typing(&context.http).await;
                        }
                    }
                }
                Err(why) => {
                    eprintln!("Error sending chore message: {:?}", why);
                }
            }
        } else if message.starts_with("/moon") {
            let line = match get_moon_phase().await {
                Ok(phase) => "The moon is ".to_owned() + phase.to_string().as_str() + "% full.",
                Err(_why) => "Vibes are off. Cannot determine the moon.".to_owned(),
            };

            let res = MessageBuilder::new().push(line).build();
            if let Err(why) = msg.channel_id.say(&context.http, &res).await {
                eprintln!("Error sending moon message: {:?}", why);
            }
        } else if message.starts_with("/stats") {
            let d: HashMap<UserId, i32> = read_stats_map();
            let count = d.get(&msg.author.id);

            let mut line = match count {
                Some(c) => format!("You have completed {} chores.", c),
                None => "You have not done any chores! This upsets Choretortle.".to_owned(),
            };
            let asterisk_user_id = env::var("ASTERISK_USER_ID").expect("userid");
            if msg.author.id == UserId::from_str(asterisk_user_id.as_str()).unwrap() {
                line = format!(
                    "You have completed {:?}* chores.\n(* with the help of -ads).",
                    count.unwrap()
                );
            }
            let res = MessageBuilder::new().push(line).build();
            if let Err(why) = msg.channel_id.say(&context.http, &res).await {
                eprintln!("Error sending moon message: {:?}", why);
            }
        } else if message.starts_with("/add") {
            let t = &message[6..];
            let fname;
            if message.starts_with("/add_c") {
                fname = "common".to_owned();
            } else if message.starts_with("/add_u") {
                fname = "uncommon".to_owned();
            } else if message.starts_with("/add_r") {
                fname = "rare".to_owned();
            } else if message.starts_with("/add_s") {
                fname = "spicy".to_owned();
            } else {
                fname = "".to_owned();
            }

            if !fname.is_empty() {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(format!("chores/{}", fname))
                    .unwrap();
                let new_chore = message
                    .split(" ")
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()[1..]
                    .join(" ");
                file.write_all(format!("{}\n", new_chore).as_bytes());
            }
        }
        if message.contains("choretle") || message.contains("choretortle") {
            eprintln!("reacting");
            msg.react(&context.http, ReactionType::from('👀')).await;
        }
    }

    async fn ready(&self, context: Context, ready: Ready) {
        eprintln!("{} is connected", ready.user.name);

        match &self.mode {
            Mode::Say => match &self.args {
                Some(s) => {
                    self.allowed_channel.broadcast_typing(&context.http).await;
                    sleep(Duration::from_millis(2000));
                    let res = MessageBuilder::new().push(s).build();

                    if let Err(why) = self.allowed_channel.say(&context.http, &res).await {
                        eprintln!("Error sending message: {:?}", why);
                    }
                    std::process::exit(0);
                }
                None => {
                    eprintln!("need args");
                    std::process::exit(0)
                }
            },
            Mode::Script => {
                let lines = script();
                for line in lines {
                    self.allowed_channel.broadcast_typing(&context.http).await;
                    sleep(Duration::from_millis(10000));
                    self.allowed_channel.broadcast_typing(&context.http).await;
                    sleep(Duration::from_millis(10000));
                    let res = MessageBuilder::new().push(line).build();
                    if let Err(why) = self.allowed_channel.say(&context.http, &res).await {
                        eprintln!("Error sending message: {:?}", why);
                    }

                    self.allowed_channel.broadcast_typing(&context.http).await;
                    sleep(Duration::from_millis(10000));
                    self.allowed_channel.broadcast_typing(&context.http).await;
                    sleep(Duration::from_millis(10000));
                }

                let custom_react_msg = 893900558933590076;
                let m = self
                    .allowed_channel
                    .message(&context, custom_react_msg)
                    .await
                    .unwrap();
                m.react(&context.http, ReactionType::from('👀')).await;

                std::process::exit(0)
            }
            Mode::Calc => {
                let messages = fetch_all_messages(self.allowed_channel, &context).await;
                eprintln!("{}", messages.len());

                //                 don't have permission to pin it seems :/
                //                 for m in &messages {
                //                     if m.content.contains("LONG LIVE THE NEW FLESH")
                //                         || m.content
                //                             .contains("You can now see how many chores you've completed with")
                //                         || m.content
                //                             .contains("will output the current phase of Earth's moon")
                //                         || m.content.contains("- Ascend. -")
                //                     {
                //                         m.pin(&context.http).await;
                //                     }
                //                 }

                let mut ascended = HashSet::new();

                for i in 0..messages.len() {
                    if i == messages.len() {
                        break;
                    }
                    let m = &messages[i];
                    if m.content.contains("-  Ascend.  -") {
                        let prev = &messages[i + 1];
                        ascended.insert(&prev.author);
                        eprintln!("{} ascended", &prev.author);
                    }
                }

                let count: HashMap<UserId, i32> = count_message_stats(&messages);
                eprintln!("ascended {}", ascended.len() as f32);
                eprintln!("total {}", count.len() as f32);
                // eprintln!( "ascended  / total = {}", ascended.len() as f32 / count.len() as f32);

                fs::write("chores/stats", serde_json::to_string(&count).unwrap())
                    .expect("error with stats file");
                eprintln!("done");

                std::process::exit(0)
            }
            _ => {
                // normal
            }
        }
    }
}

async fn fetch_all_messages(allowed_channel: ChannelId, context: &Context) -> Vec<Message> {
    let mut messages = Vec::new();
    let mut new_messages = Vec::new();
    let mut oldest_message_id = allowed_channel
        .messages(&context.http, |ret| ret.limit(1))
        .await
        .unwrap()
        .first()
        .unwrap()
        .id;
    let mut is_more = true;

    while is_more {
        async {
            new_messages = allowed_channel
                .messages(&context.http, |retriever| {
                    retriever.before(oldest_message_id).limit(100)
                })
                .await
                .unwrap();
        }
        .await;

        if new_messages.len() < 100 {
            is_more = false;
            break;
        }
        oldest_message_id = new_messages.last().unwrap().id;
        messages.append(&mut new_messages);
    }
    messages
}

fn count_message_stats(messages: &Vec<Message>) -> HashMap<UserId, i32> {
    let mut count: HashMap<UserId, i32> = HashMap::new();
    for i in 0..messages.len() {
        if i >= messages.len() - 1 {
            break;
        }
        let mess = &messages[i];
        if mess.author.name.eq("Choretortle, Giver of Chores")
            || mess.author.name.eq("Choretle")
            || mess.author.name.eq("choretle_dev")
        {
            let prev = &messages[i + 1];

            let rarity;
            if mess.content.contains("diff") {
                rarity = RarityType::Spicy;
            } else if mess.content.contains("md") {
                rarity = RarityType::Rare;
            } else if mess.content.contains("cs") {
                rarity = RarityType::Uncommon;
            } else {
                rarity = RarityType::Common;
            }
            match count.get_mut(&prev.author.id) {
                Some(c) => *c += 1,
                None => {
                    count.insert(prev.author.id, 1);
                }
            }
        }
    }
    count
}

fn read_stats_map() -> HashMap<UserId, i32> {
    serde_json::from_str(
        fs::read_to_string("chores/stats")
            .expect("error reading file")
            .as_str(),
    )
    .unwrap()
}

fn increment_count(user: UserId) -> i32 {
    let mut d = read_stats_map();
    let n = match d.get(&user) {
        Some(i) => *i + 1,
        None => 1,
    };
    d.insert(user, n);
    fs::write("chores/stats", serde_json::to_string(&d).unwrap())
        .expect("error writing to update stats file");
    n
}

#[derive(Clone, Debug, Deserialize)]
struct Data {
    time: String,
    phase: f32,
    age: f32,
    diameter: f32,
    distance: f32,
}
async fn get_moon_phase() -> Result<f32, Box<dyn std::error::Error>> {
    let body: Vec<Data> =
        reqwest::get("https://svs.gsfc.nasa.gov/vis/a000000/a004800/a004874/mooninfo_2021.json")
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

    let year_start: i64 = DateTime::parse_from_rfc3339("2021-01-01T00:00:00+00:00")?.timestamp();
    let now = chrono::Utc::now().timestamp();
    let idx = (now - year_start) / 3600;
    Ok(body[idx as usize].clone().phase)
}

fn collect_f(fname: &str) -> Vec<String> {
    fs::read_to_string("chores/".to_owned() + fname)
        .expect("error reading file")
        .split("\n")
        .map(|s| s.to_owned())
        .collect()
}

fn remove_line(fname: &str, new_list: Vec<String>) {
    match fs::write("chores/".to_owned() + fname, new_list.join("\n")) {
        Ok(res) => res,
        Err(why) => {
            eprintln!("error writing new list: {}", why);
        }
    }
}

fn is_decorated(line: &String) -> bool {
    line.contains("||") || line.contains("~~")
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new().configure(|c| c.prefix("~")); // set the bot's prefix to "~"

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let allowed_channel =
        ChannelId::from_str(&env::var("ALLOWED_CHANNEL_ID").expect("allowed_channel_id")).unwrap();

    let args: Vec<String> = env::args().collect();
    let mut handler = Handler {
        allowed_channel,
        args: None,
        mode: Mode::Normal,
    };
    if args.len() > 1 {
        if args[1].eq("say") {
            if args[2..].len() > 0 {
                handler = Handler {
                    allowed_channel,
                    args: Some(args[2..].join(" ")),
                    mode: Mode::Say,
                }
            } else {
                eprintln!("need args to say!");
                return;
            }
        } else if args[1].eq("script") {
            handler = Handler {
                allowed_channel,
                args: None,
                mode: Mode::Script,
            }
        } else if args[1].eq("calc") {
            handler = Handler {
                allowed_channel,
                args: Some(args[2..].join(" ")),
                mode: Mode::Calc,
            }
        } else {
            handler = Handler {
                allowed_channel,
                args: None,
                mode: Mode::Normal,
            };
        }
    }

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

fn script() -> [String; 0] {
    // let user_one_id: u64 = u64::from_str(env::var("USER_ONE_ID").expect("id").as_str()).unwrap();
    // let user_one = UserId::from(user_one_id);
    // let user_two_id: u64 = u64::from_str(env::var("USER_TWO_ID").expect("id").as_str()).unwrap();
    // let user_two = UserId::from(user_two_id);
    // let user_three_id: u64 =
    //     u64::from_str(env::var("USER_THREE_ID").expect("id").as_str()).unwrap();
    // let user_three = UserId::from(user_three_id);

    [
        // "```Chortle starts to shake and bounce unnaturally. Eventually rising about as high as an altar.
        // Time is a worm and has slowed as such.
        // The paper replacement hatch blows off and flies out the window into a darkened sky. From inside pours a bloodblack liquid. It covers the floor. Reams of Chortle’s paper are now ruined.
        // A metallic head pokes from the hatch reflecting light from elsewhere.
        // The body emerges. The face screams.

        // LONG LIVE THE NEW FLESH

        // The entity looks almost identical to its predecesor, but quicksilver-metallic. The shimmer is unreal. It looks at you in the eyes. You feel a liquid pooling at your ankles, but canot look away. As you stare with the entity moments turn to whole hours. Time, which was just at a crawl, is now rocketing like a meteor on it's way to annihillate the frat house.
        // The shell of the old Chortle shrivels and oxidizes. The floor has dried whith a crimson stain. The violin stops. You take a deep breath. It is a Friday. Chortle 2.0 stands in a pile of Rust.".to_owned(),
        // format!("scanning {}...", user_one.mention()),
        // format!("scanning {}...", user_two.mention()),
        // format!("scanning {}...", user_three.mention()),
        // "*Choretortle begins ticking*".to_owned(),
        // "You can now see how many chores you've completed with `/stats`.".to_owned()
        // format!("```diff
        // ----- analysis complete -----
        // ---
        // --- -̶͛̚ ̶̛̛ ̴̉̌ ̴͐̊ ̴̊̀ ̷͂̾ ̵͙͑ ̷͆͛ ̶͗̐.̸͒͒ ̸̑͠ ̵̆̓ ̷̈́̉ ̶̽̓ ̵̀̾ ̴̄̐ ̶̑̾ ̷͗͘ ̵̌͗:̵́͊ ̶̈̐ ̷̓̑ ̴̒͝ ̶̆̏-̸̿ͅ completed by Real_Atomsk during a full moon.
        // -- this will be remembered --
        // --------------------------------
        // ```"),
        // "new command available!".to_owned(),
        // "/moon will output the current phase of Earth's moon".to_owned(),
    ]
}
