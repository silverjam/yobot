//!
//! `SlackHandler` handles all the interactions with the slack API. It connects a
//! persistent socket to the Real Time API and listens for all the events. Events are
//! communicated back to users of this method by a closure given on the initializer.
//!
//! # Example
//!
//! ```no_run
//! # fn main() {
//! let mut handler = SlackHandler::new(|message, _| { println!("Yo {}", message); });
//! handler.login_and_run(token);
//! # }
//! ```
extern crate slack;

use regex::Regex;
use slack::Event;
use listener::Message;

pub struct SlackHandler<F> {
    event_handler: F,
    addresser: Regex,
}

impl<F> SlackHandler<F>
where
    F: Fn(&Message, &slack::RtmClient),
{
    pub fn new(event_handler: F) -> SlackHandler<F> {
        SlackHandler {
            event_handler: event_handler,
            addresser: Regex::new(".").unwrap(),
        }
    }

    pub fn login_and_run(&mut self, token: String, bot_name: String) {
        match slack::RtmClient::login(&token) {
            Err(err) => panic!("Error: {}", err),
            Ok(client) => {
                let bot_id = client
                    .start_response()
                    .users
                    .as_ref()
                    .and_then(|users| {
                        users.iter().find(|user| match user.name.as_ref() {
                            None => false,
                            Some(name) => &bot_name == name,
                        })

                    })
                    .and_then(|bot| bot.id.as_ref())
                    .expect("couldn't find bot from bot name");

                let addresser_regex = format!(r"^(<@{}>|{})?[:,\s]\s*", bot_id, bot_name);
                self.addresser = Regex::new(&addresser_regex).unwrap();

                if let Err(err) = client.run(self) {
                    panic!("Error: {}", err);
                }
            }
        };
    }

    fn parse_message(&self, raw: &str) -> (bool, String) {
        let mut message = raw.clone();
        let mut is_addressed = false;

        if let Some(captures) = self.addresser.captures(raw) {
            is_addressed = true;

            let prefix_len = captures.get(0).unwrap().end();
            let message_len = message.len();
            unsafe {
                message = message.slice_unchecked(prefix_len, message_len);
            }
        }

        (is_addressed, message.to_string())
    }
}

impl<F> slack::EventHandler for SlackHandler<F>
where
    F: Fn(&Message, &slack::RtmClient),
{
    fn on_event(&mut self, cli: &slack::RtmClient, event: Event) {
        match event {
            slack::Event::Message(box_message) => {
                match *box_message {
                    slack::Message::Standard(msg) => {
                        let text = msg.text.unwrap_or("".to_owned());
                        let (is_addressed, message) = self.parse_message(&text);
                        let channel = msg.channel.unwrap_or("".to_owned());
                        let message = Message {
                            channel: channel.clone(),
                            is_addressed: is_addressed,
                            text: message,
                        };
                        (self.event_handler)(&message, cli);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn on_connect(&mut self, _cli: &slack::RtmClient) {
        println!("RTM API connected")
    }

    fn on_close(&mut self, _cli: &slack::RtmClient) {
        println!("RTM API disconnected")
    }
}
