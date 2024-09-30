use clap::{ArgMatches, Command};

use crate::configure::{self, Config};

pub fn cmd_init() -> Command {
    clap::Command::new("config")
        .about("config")
        .subcommand(config_show_cmd())
}

fn config_show_cmd() -> Command {
    clap::Command::new("show")
        .about("show some info ")
        .subcommand(config_show_info_cmd())
        .subcommand(config_show_all_cmd())
}

fn config_show_info_cmd() -> Command {
    clap::Command::new("info").about("show info")
}

fn config_show_all_cmd() -> Command {
    clap::Command::new("all").about("show all ")
}

pub fn cmd_exec(matches: &ArgMatches) {
    if let Some(config) = matches.subcommand_matches("config") {
        if let Some(show) = config.subcommand_matches("show") {
            match show.subcommand_name() {
                Some("current") => {
                    let current = configure::get_config().expect("get current configure error!");
                    let yml =
                        serde_yaml::to_string(&current).expect("pars configure to yaml error!");
                    println!("{}", yml);
                }
                Some("default") => {
                    let config = Config::default();
                    let yml = serde_yaml::to_string(&config);
                    match yml {
                        Ok(y) => {
                            println!("{}", y);
                        }
                        Err(e) => {
                            log::error!("{}", e);
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(gen_config) = config.subcommand_matches("gendefault") {
            let mut file = String::from("");
            if let Some(path) = gen_config.get_one::<String>("filepath") {
                file.push_str(path);
            } else {
                file.push_str("config_default.yml")
            }
            if let Err(e) = configure::generate_default_config(file.as_str()) {
                log::error!("{}", e);
                return;
            };
            println!("{} created!", file);
        }
    }
}
