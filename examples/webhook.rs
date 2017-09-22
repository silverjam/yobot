extern crate slack;
extern crate regex;
extern crate yobot;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use std::env;
use regex::Regex;
use yobot::listener::{MessageListener, Message};
use yobot::Yobot;

struct WebhookListener {
    regex: Regex,
    client: reqwest::Client,
    url: String,
}

impl WebhookListener {
    pub fn new(client: reqwest::Client) -> WebhookListener {
        WebhookListener {
            regex: Regex::new(r".+").unwrap(),
            client: client,
            url: "http://your-site.biz/api/v1/message/create".to_owned(),
        }
    }
}

#[derive(Serialize)]
struct NewEntry {
    title: String,
    text: String,
}

impl MessageListener for WebhookListener {
    fn help(&self) -> String {
        String::from("say my name, and I'll post your message")
    }

    fn re(&self) -> &Regex {
        &self.regex
    }

    fn only_when_addressed(&self) -> bool {
        true
    }

    fn handle(&self, message: &Message, cli: &slack::RtmClient) {
        let new_entry = NewEntry {
            title: "webhook_listener".to_owned(),
            text: message.text.clone(),
        };

        let msg = match self.client
            .post(&self.url)
            .unwrap()
            .json(&new_entry)
            .unwrap()
            .send() {
            Ok(res) => {
                if res.status().is_success() {
                    "successfully posted"
                } else {
                    "failed to posted"
                }
            }
            Err(_) => "failed to posted",
        };
        match cli.sender().send_message(&message.channel, msg) {
            Ok(_) => {}
            Err(err) => println!("failed to send message: {}", err),
        }
    }
}

fn main() {
    let token = match env::var("SLACK_BOT_TOKEN") {
        Ok(token) => token,
        Err(_) => panic!("Failed to get SLACK_BOT_TOKEN from env"),
    };
    let bot_name = match env::var("SLACK_BOT_NAME") {
        Ok(bot_name) => bot_name,
        Err(_) => panic!("Failed to get SLACK_BOT_NAME from env"),
    };


    let client = reqwest::Client::new().unwrap();
    let listener = WebhookListener::new(client);
    let mut yobot = Yobot::new();
    yobot.add_listener(listener);
    yobot.connect(token, bot_name);
}
