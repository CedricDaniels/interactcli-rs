use crate::commons::CommandCompleter;
use crate::commons::SubCmd;
use crate::configure::{get_config, get_config_file_path};
use crate::configure::{set_config_file_path, set_config_from_file};
use crate::interact::{self, INTERACT_STATUS};
use crate::request::Request;

use clap::{Arg, ArgAction, ArgMatches, Command as clap_Command};

use lazy_static::lazy_static;
use std;
use sysinfo::{PidExt, System, SystemExt};

use super::{
    argdaemon, cmd_spinoff_sample, cmdconfig, cmdloop, cmdmultilevel, cmdserver, cmdtask,
    cmdusedifflogger, json, requestsample,
};

lazy_static! {
    static ref CLIAPP: clap::Command = clap::Command::new("interact-rs")
        .version("1.0")
        .author("Shiwen Jia. <jiashiwen@gmail.com>")
        .about("command line sample")
        .arg_required_else_help(true)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
        )
        .arg(argdaemon::arg_init())
        .arg(
            Arg::new("interact")
                .short('i')
                .long("interact")
                .conflicts_with("daemon")
                .action(ArgAction::SetTrue)
                .help("run as interact mod")
        )
        .subcommand(requestsample::cmd_init())
        .subcommand(cmdconfig::cmd_init())
        .subcommand(cmdmultilevel::cmd_init())
        .subcommand(cmdtask::cmd_init())
        .subcommand(cmdloop::cmd_init())
        .subcommand(cmdusedifflogger::cmd_init())
        .subcommand(cmdserver::cmd_init())
        .subcommand(cmd_spinoff_sample::cmd_init())
        .subcommand(json::cmd_init());
    static ref SUBCMDS: Vec<SubCmd> = subcommands();
}

pub fn run_app() {
    let matches = CLIAPP.clone().get_matches();
    if let Some(c) = matches.get_one::<String>("config") {
        // if let Some(c) = matches.value_of("config") {
        println!("config path is:{}", c);
        set_config_file_path(c.to_string());
    }
    cmd_match(&matches);
}

pub fn run_from(args: Vec<String>) {
    match clap_Command::try_get_matches_from(CLIAPP.to_owned(), args.clone()) {
        Ok(matches) => {
            cmd_match(&matches);
        }
        Err(err) => {
            err.print().expect("Error writing Error");
        }
    };
}

// 获取全部子命令，用于构建commandcompleter
pub fn all_subcommand(app: &clap_Command, beginlevel: usize, input: &mut Vec<SubCmd>) {
    let nextlevel = beginlevel + 1;
    let mut subcmds = vec![];
    for iterm in app.get_subcommands() {
        subcmds.push(iterm.get_name().to_string());
        if iterm.has_subcommands() {
            all_subcommand(iterm, nextlevel, input);
        } else {
            if beginlevel == 0 {
                all_subcommand(iterm, nextlevel, input);
            }
        }
    }
    let subcommand = SubCmd {
        level: beginlevel,
        command_name: app.get_name().to_string(),
        subcommands: subcmds,
    };
    input.push(subcommand);
}

pub fn get_command_completer() -> CommandCompleter {
    CommandCompleter::new(SUBCMDS.to_vec())
}

fn subcommands() -> Vec<SubCmd> {
    let mut subcmds = vec![];
    all_subcommand(&CLIAPP, 0, &mut subcmds);
    subcmds
}

pub fn process_exists(pid: &u32) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (syspid, _) in sys.processes() {
        if syspid.as_u32().eq(pid) {
            return true;
        }
    }
    return false;
}

fn cmd_match(matches: &ArgMatches) {
    if let Some(c) = matches.get_one::<String>("config") {
        // if let Some(c) = matches.value_of("config") {
        set_config_file_path(c.to_string());
        set_config_from_file(&get_config_file_path());
    } else {
        set_config_from_file("");
    }
    let config = get_config().unwrap();
    let server = config.server;
    let req = Request::new(server.clone());

    argdaemon::daemon_command(matches);

    interact_command(matches);

    // 测试 log 写入不同文件
    cmdusedifflogger::cmd_exec(matches);
    cmdloop::cmd_exec(matches);

    requestsample::cmd_exec(matches);
    cmdtask::cmd_exec(req, matches);
    cmdserver::cmd_exec(matches);
    cmdconfig::cmd_exec(matches);
    cmd_spinoff_sample::cmd_exec(matches);
    json::cmd_exec(matches);
}

fn interact_command(matches: &ArgMatches) {
    if matches.get_flag("interact") {
        if !INTERACT_STATUS.load(std::sync::atomic::Ordering::SeqCst) {
            interact::run();
        }
    }
}
