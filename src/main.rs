mod common;
mod commands;

use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("rsncp")
        .about("Rust implementation of Network Copy")
        .version("0.0.0")
        .author("Jeff Parent")
        .subcommand(
            App::new("listen")
                .about("runs in listener mode, waits for connection")
        )
        .subcommand(
            App::new("send")
                .about("sends files to a listener mode receiver")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("DEST")
                        .help("IP / Domain Name of listening connection")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::with_name("FILES")
                        .help("Files / Directories to send")
                        .required(true)
                        .takes_value(true)
                        .multiple(true)
                )
        )
        .subcommand(
            App::new("push")
                .about("broadcasts for clients, waits for connection")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("FILE")
                        .help("Files to send")
                        .required(true)
                        .multiple(true)
                )
        )
        .subcommand(
            App::new("poll")
                .about("waits for braodcast, connects to braodcaster")
        )
        .get_matches();

    match matches.subcommand() {
        ("listen", _) => println!("Listen not implemented"),
        ("send", Some(send_matches)) => {
            let dst = send_matches.value_of("DEST").unwrap();
            let files = send_matches
                .values_of("FILES")
                .unwrap()
                .collect::<Vec<_>>()
                .iter()
                .map(|s| String::from(*s))
                .collect::<Vec<String>>();

            commands::do_send(String::from(dst), files);
        },
        ("push", _) => println!("Push not implemented"),
        ("poll", _) => println!("Poll not implemented"),
        ("", _) => println!("No subcommand issued"),
        _ => unreachable!(),
    }
}
