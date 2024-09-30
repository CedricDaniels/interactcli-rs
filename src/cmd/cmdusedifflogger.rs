use clap::{ArgMatches, Command};

pub fn cmd_init() -> Command {
    clap::Command::new("uselog")
        .about("use diffrent target log")
        .subcommand(new_use_sys_log_cmd())
        .subcommand(new_use_business_log_cmd())
}

pub fn new_use_sys_log_cmd() -> Command {
    clap::Command::new("syslog").about("append to syslog")
}

pub fn new_use_business_log_cmd() -> Command {
    clap::Command::new("businesslog").about("append to business log")
}

pub fn cmd_exec(matches: &ArgMatches) {
    if let Some(ref log) = matches.subcommand_matches("uselog") {
        println!("use log");
        if let Some(_) = log.subcommand_matches("syslog") {
            log::info!(target:"syslog","Input sys log");
        }

        if let Some(_) = log.subcommand_matches("businesslog") {
            log::info!(target:"businesslog","Input business log");
        }
    }
}
