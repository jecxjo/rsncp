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
use common::version;

fn main() {
    let matches = App::new("rsncp")
        .about("Rust implementation of Network Copy")
        .version(version().as_str())
        .author("Jeff Parent")
        .subcommand(
            App::new("listen")
                .about("runs in listener mode, waits for connection")
                .arg(
                    Arg::with_name("legacy")
                        .short("l")
                        .help("Enables legacy support (no compression)")
                )
        )
        .subcommand(
            App::new("send")
                .about("sends files to a listener mode receiver")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("legacy")
                        .short("l")
                        .help("Enables legacy support (no compression)")
                )
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
                    Arg::with_name("legacy")
                        .short("l")
                        .help("Enables legacy support (no compression)")
                )
                .arg(
                    Arg::with_name("FILES")
                        .help("Files to send")
                        .required(true)
                        .multiple(true),
                ),
        )
        .subcommand(App::new("poll")
            .about("waits for braodcast, connects to braodcaster")
            .arg(
                Arg::with_name("legacy")
                .short("l")
                .help("Enables legacy support (no compression)")
            )
        )
        .get_matches();

    let results = match matches.subcommand() {
        ("listen", Some(listen_matches)) => {
            let legacy = listen_matches.is_present("legacy");

            if legacy { println!("[#] LEGACY MODE"); }

            println!("[#] Listening for files from Sender");
            let listen = Listen::new(legacy);
            listen.do_listen()
        }
        ("send", Some(send_matches)) => {
            let legacy = send_matches.is_present("legacy");
            let dst = send_matches.value_of("DEST").unwrap();
            let files = send_matches
                .values_of("FILES")
                .unwrap()
                .collect::<Vec<_>>()
                .iter()
                .map(|s| String::from(*s))
                .collect::<Vec<String>>();

            if legacy { println!("[#] LEGACY MODE"); }
            println!("[#] Sending files to Listener");

            let send = Send::new(legacy, String::from(dst), files);
            send.do_send()
        }
        ("poll", Some(poll_matches)) => {
            let legacy = poll_matches.is_present("legacy");

            if legacy { println!("[#] LEGACY MODE"); }
            println!("[#] Polling for Broadcaster");

            let poll = Poll::new(legacy);
            poll.do_poll()
        }
        ("push", Some(push_matches)) => {
            let legacy = push_matches.is_present("legacy");

            if legacy { println!("[#] LEGACY MODE"); }
            println!("[#] Pushing to Poller");
            let files = push_matches
                .values_of("FILES")
                .unwrap()
                .collect::<Vec<_>>()
                .iter()
                .map(|s| String::from(*s))
                .collect::<Vec<String>>();

            let push = Push::new(legacy, files);
            push.do_push()
        }
        ("", _) => {
            println!("[#] Listening for files from Sender");
            let listen = Listen::new(false);
            listen.do_listen()
        }
        _ => unreachable!(),
    };

    match results {
        Err(e) => println!("[!] {}", e),
        Ok(_) => println!("[#] done"),
    }
}
