use ignore_result::Ignore;
use serenity::{
    client::{Client, Context, EventHandler},
    model::{channel::Message, event::MessageUpdateEvent},
};

pub struct BumbiBot;

impl BumbiBot {
    pub fn run() {
        let mut client = Client::new(
            &std::env::var("TOKEN").expect("$TOKEN not supplied"),
            BumbiBot::new(),
        )
        .expect("Failed to create a serenity client");

        if let Err(err) = client.start() {
            eprintln!("Failed while running client: {:?}", err);
        }
    }
    fn new() -> Self {
        Self
    }
}

impl EventHandler for BumbiBot {
    fn message(&self, _: Context, message: Message) {
        if let Some(channel) = message.channel() {
            if channel.id().0 == 552173697847263241 {
                if !is_ctf_flag(&message.content) {
                    message.delete().ignore();
                } else {
                    message.react('👌').ignore();
                }
            }
        }
    }
    fn message_update(&self, _: Context, message: MessageUpdateEvent) {
        if message.channel_id == 552173697847263241 {
            if let Some(content) = message.content {
                if !is_ctf_flag(&content) {
                    message.channel_id.delete_message(message.id).ignore();
                }
            }
        }
    }
}

fn is_ctf_flag(s: &str) -> bool {
    s.chars().enumerate().try_for_each(|(i, c)| {
        match i {
            0 => if c == 'S' { Ok(()) } else { Err(()) }
            1 => if c == 'S' { Ok(()) } else { Err(()) }
            2 => if c == 'M' { Ok(()) } else { Err(()) }
            3 => if c == '{' { Ok(()) } else { Err(()) }
            _ if c == '}' => if i == s.len() - 1 { Ok(()) } else { Err(()) }
            _ if c >= 'a' && c <= 'z' => Ok(()),
            _ if c >= 'A' && c <= 'Z' => Ok(()),
            _ if c == '_' => Ok(()),
            _ => { Err(()) },
        }
    }).is_ok()
}
