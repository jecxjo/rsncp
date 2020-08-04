#[macro_use]
extern crate lazy_static;
extern crate socket2;

mod commands;
mod common;

use clap::{App, AppSettings, Arg};
use commands::listen::Listen;
use commands::poll::Poll;
use commands::push::Push;
use commands::send::Send;

fn main() {
    let matches = App::new("rsncp")
        .about("Rust implementation of Network Copy")
        .version("0.0.0")
        .author("Jeff Parent")
        .subcommand(App::new("listen").about("runs in listener mode, waits for connection"))
        .subcommand(
            App::new("send")
                .about("sends files to a listener mode receiver")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("DEST")
                        .help("IP / Domain Name of listening connection")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("FILES")
                        .help("Files / Directories to send")
                        .required(true)
                        .takes_value(true)
                        .multiple(true),
                ),
        )
        .subcommand(
            App::new("push")
                .about("broadcasts for clients, waits for connection")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("FILE")
                        .help("Files to send")
                        .required(true)
                        .multiple(true),
                ),
        )
        .subcommand(App::new("poll").about("waits for braodcast, connects to braodcaster"))
        .get_matches();

    let results = match matches.subcommand() {
        ("listen", _) => {
            let listen = Listen {};
            listen.do_listen()
        }
        ("send", Some(send_matches)) => {
            let dst = send_matches.value_of("DEST").unwrap();
            let files = send_matches
                .values_of("FILES")
                .unwrap()
                .collect::<Vec<_>>()
                .iter()
                .map(|s| String::from(*s))
                .collect::<Vec<String>>();

            let send = Send::new(String::from(dst), files);
            send.do_send()
        }
        ("poll", _) => {
            let poll = Poll {};
            poll.do_poll()
        }
        ("push", Some(push_matches)) => {
            let files = push_matches
                .values_of("FILES")
                .unwrap()
                .collect::<Vec<_>>()
                .iter()
                .map(|s| String::from(*s))
                .collect::<Vec<String>>();

            let push = Push::new(files);
            push.do_push()
        }
        ("", _) => {
            let listen = Listen {};
            listen.do_listen()
        }
        _ => unreachable!(),
    };

    match results {
        Err(e) => println!("[!] {}", e),
        Ok(_) => println!("[#] done"),
    }
}
