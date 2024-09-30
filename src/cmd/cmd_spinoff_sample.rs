use std::{thread::sleep, time::Duration};

use clap::{ArgMatches, Command};
use spinoff::{spinners, Color, Spinner};

pub fn cmd_init() -> Command {
    clap::Command::new("spinoff_sample").about("spinoff_sample")
}

pub fn cmd_exec(matches: &ArgMatches) {
    if let Some(spinoff_sample) = matches.subcommand_matches("spinoff_sample") {
        let mut spinner = Spinner::new(spinners::Dots, "Loading...", Color::Blue);
        // sleep(Duration::from_secs(3));
        let rt = tokio::runtime::Runtime::new().unwrap();
        let async_req = async {
            sleep(Duration::from_secs(3));
        };
        rt.block_on(async_req);

        spinner.success("Done!");
    }
}