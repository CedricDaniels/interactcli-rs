use clap::{ArgMatches, Command};

use crate::request::req;

pub fn cmd_init() -> Command {
    clap::Command::new("requestsample")
        .about("requestsample")
        .subcommand(get_baidu_cmd())
}

pub fn get_baidu_cmd() -> Command {
    clap::Command::new("baidu").about("request www.baidu.com")
}

pub fn cmd_exec(matches: &ArgMatches) {
    if let Some(ref matches) = matches.subcommand_matches("requestsample") {
        if let Some(_) = matches.subcommand_matches("baidu") {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let async_req = async {
                let result = req::get_baidu().await;
                match result {
                    Ok(r) => {
                        println!("{}", r);
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                };
            };
            rt.block_on(async_req);
        };
    }
}
