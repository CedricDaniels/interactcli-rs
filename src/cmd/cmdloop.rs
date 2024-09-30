use chrono::Local;
use clap::{ArgMatches, Command};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};

pub fn cmd_init() -> Command {
    clap::Command::new("loop").about("loop")
}

pub fn cmd_exec(matches: &ArgMatches) {
    if let Some(ref _matches) = matches.subcommand_matches("loop") {
        let term = Arc::new(AtomicBool::new(false));
        let sigint_2 = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).unwrap();
        signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&sigint_2)).unwrap();
        loop {
            if sigint_2.load(Ordering::Relaxed) {
                println!("{}", "singint signal recived");
                break;
            }

            thread::sleep(Duration::from_millis(1000));
            if term.load(Ordering::Relaxed) {
                println!("{:?}", term);
                break;
            }
            let dt = Local::now();
            let _ = fs::write("timestamp", dt.timestamp_millis().to_string());
        }
    }
}
