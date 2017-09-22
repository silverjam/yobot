# yobot
__Note__: This was forked by me (@stuartnelson3) for the purpose of updating
and releasing this on crates.io. All credit goes to the original creator,
Martin Conte Mac Donell <martin@lyft.com>.

Yobot is an extensible slack bot. You can add listener that define a regex in order to
handle real time events on a slack channel.

You need to create a bot, then supply its api token and name when starting
`Yobot`. Check out the examples in `examples/`

## Listeners

Listeners provide a `Regex` which the main loop uses to check whether the listener is interested.
If the regex matches, `handle` is called on the handler with the Message and RtmClient. The
listener can then do some work and use the client to send its response.

Note that a panic on a listener (or a crash) will crash the entire client.

Implementing a `MessageListener` enables responding to `slack::Message`s. There
are currently very few requirements to creating a handler. The
[`handle`](#method.handle) function receives a `slack::Message` and a `slack::RtmClient`.
The listener is responsible for testing whether it's interested in replying by
defining a regular expression on [`re`](#method.re)
Optionally call `cli.send_message` to send a response.

### Example

A simple echo handler might look something like the following:

```rust
use regex::Regex;
use yobot::listener::{MessageListener, Message};

pub struct EchoListener {
    regex: Regex
}

impl EchoListener {
    pub fn new() -> EchoListener {
        EchoListener {
            regex: Regex::new(r".").unwrap()
        }
    }
}

impl MessageListener for EchoListener {
    fn help(&self) -> &str {
        "echo"
    }

    fn re(&self) -> &Regex {
        &self.regex
    }

    fn handle(&self, message: &Message, cli: &slack::RtmClient) {
        let _ = cli.sender().send_message(&message.channel, &message.text);
    }
}
```

## Yobot

Yobot is the main struct of the bot. Add a bunch of listeners and you call `connect` to connect
the real time API and start listening for messages.

### Example

```rust
fn main() {
    let token = match env::var("SLACK_BOT_TOKEN") {
        Ok(token) => token,
        Err(_) => panic!("Failed to get SLACK_BOT_TOKEN from env"),
    };
    let bot_name = match env::var("SLACK_BOT_NAME") {
        Ok(bot_name) => bot_name,
        Err(_) => panic!("Failed to get SLACK_BOT_NAME from env"),
    };

    let listener = EchoListener::new();
    let mut yobot = Yobot::new();
    yobot.add_listener(listener);
    yobot.connect(token, bot_name);
}
```
