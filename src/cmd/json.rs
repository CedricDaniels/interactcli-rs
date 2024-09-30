use clap::{Arg, ArgMatches, Command};

pub fn cmd_init() -> Command {
    clap::Command::new("json")
        .about("json tools")
        .version("0.1")
        .author("cedric")
        .subcommand(
            clap::Command::new("pretty")
                .about("json pretty")
                .version("0.1")
                .author("cedric")
                .arg(Arg::new("value").short('v').help("json string")),
        )
        .subcommand(
            clap::Command::new("minify")
                .about("json minify")
                .version("0.1")
                .author("cedric")
                .arg(Arg::new("value").short('v').help("json string")),
        )
}

pub fn cmd_exec(matches: &ArgMatches) {
    if let Some(ref matches) = matches.subcommand_matches("json") {
        if let Some(ref matches) = matches.subcommand_matches("pretty") {
            if let Some(v) = matches.get_one::<String>("value") {
                println!("{}", jsonxf::pretty_print(v).unwrap_or_default());
            } else {
                return;
            }
        }
        if let Some(ref matches) = matches.subcommand_matches("minify") {
            if let Some(v) = matches.get_one::<String>("value") {
                println!("{}", jsonxf::minimize(v).unwrap_or_default());
            } else {
                return;
            }
        }
    }
}
