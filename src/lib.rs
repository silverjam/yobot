//!
//! Slack bot that can be extended with listners.
//!
//! Yobot is an extensible slack bot. You can add listeners that define a regex in order to
//! handle real time events on a slack channel.
//!
//! Settting `SLACK_BOT_TOKEN` env variable and running `cargo run` will get you a bot,
//! just invite the bot to the channels you want on slack and start the fun.
//!
//! ## Listeners
//!
//! Listeners provide a `Regex` which the main loop uses to check whether the listener is interested.
//! If the regex matches, `handle` is called on the handler with the Message and RtmClient. The
//! listener can then do some work and use the client to send its response.
//!
//! Note that a panic on a listener (or a crash) will crash the entire client.
//!
//! ## Yobot
//!
//! Yobot is the main struct of the bot. Add a bunch of listeners and you call `connect` to connect
//! the real time API and start listening for messages.
//!
//! # Example
//!
//! ```no_run
//! fn main() {
//!     let token = match env::var("SLACK_BOT_TOKEN") {
//!         Ok(token) => token,
//!         Err(_) => panic!("Failed to get SLACK_BOT_TOKEN from env"),
//!     };
//!     let bot_name = match env::var("SLACK_BOT_NAME") {
//!         Ok(bot_name) => bot_name,
//!         Err(_) => panic!("Failed to get SLACK_BOT_NAME from env"),
//!     };
//!
//!     let listener = EchoListener::new();
//!     let mut yobot = Yobot::new();
//!     yobot.add_listener(listener);
//!     yobot.connect(token, bot_name);
//! }
//! ```
//!

extern crate regex;
extern crate slack;

pub mod listener;
pub mod slackhandler;
pub mod yobot;

pub use self::yobot::Yobot;
