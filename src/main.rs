extern crate irc;

use std::default::Default;
use irc::client::prelude::*;

fn main() {
    let server = IrcServer::new("config.json").unwrap();
    for message in server.iter_cmd() {

    }
}
