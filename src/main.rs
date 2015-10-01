extern crate irc;
extern crate time;

use std::default::Default;
use irc::client::prelude::*;

use time::SteadyTime;

use std::cell::RefCell;

struct LastMessage {
    time: SteadyTime
}

fn main() {
    let server = IrcServer::new("config.json").unwrap();
    let last_message = RefCell::new(LastMessage{time: SteadyTime::now()});

    for command in server.iter_cmd() {
        match command.unwrap() {
            cmd@Command::PRIVMSG(target, raw_message) => {
                let msg: Message = cmd.into();
                // Only if channel message
                if msg.get_source_nickname().is_none() {
                    last_message.borrow_mut().time = SteadyTime::now();
                }
            },
            _ =>
                println!("something else"),
        }
    }
}
