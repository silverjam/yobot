extern crate slack;
extern crate yobot;
extern crate regex;

use std::env;
use regex::Regex;
use yobot::listener::{MessageListener, Message};
use yobot::Yobot;

pub struct EchoListener {
    regex: Regex,
}

impl EchoListener {
    pub fn new() -> EchoListener {
        EchoListener { regex: Regex::new(r".").unwrap() }
    }
}

impl MessageListener for EchoListener {
    fn help(&self) -> String {
        String::from("`echo`: Just type anything")
    }

    fn re(&self) -> &Regex {
        &self.regex
    }

    fn only_when_addressed(&self) -> bool {
        true
    }

    fn handle(&self, message: &Message, cli: &slack::RtmClient) {
        let _ = cli.sender().send_message(&message.channel, &message.text);
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


    let mut yobot = Yobot::new();
    let listener = EchoListener::new();
    yobot.add_listener(listener);
    yobot.connect(token, bot_name);
}
