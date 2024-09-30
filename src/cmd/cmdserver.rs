
use crate::server::start;
use clap::{Arg, ArgAction, ArgMatches, Command};
use daemonize::Daemonize;
use fork::{daemon, Fork};
use std::{env, fs, process};
use std::fs::File;

pub fn cmd_init() -> Command {
    clap::Command::new("server")
        .about("server")
        .subcommand(server_start_byfork())
        .subcommand(server_start_bydaemonize())
}

pub fn server_start_byfork() -> Command {
    clap::Command::new("byfork")
        .about("start daemon by fork crate")
        .arg(
            Arg::new("daemon")
                .short('d')
                .long("daemon")
                .action(ArgAction::SetTrue)
                .help("start as daemon")
                .required(false),
        )
}
pub fn server_start_bydaemonize() -> Command {
    clap::Command::new("bydaemonize")
        .about("start daemon by daemonize crate")
        .arg(
            Arg::new("daemon")
                .short('d')
                .long("daemon")
                .action(ArgAction::SetTrue)
                .help("start as daemon")
                .required(false),
        )
}



pub fn cmd_exec(matches: &ArgMatches) {
    if let Some(server) = matches.subcommand_matches("server") {
        if let Some(startbyfork) = server.subcommand_matches("byfork") {
            println!("start by fork");
            if startbyfork.get_flag("daemon") {
                let args: Vec<String> = env::args().collect();
                if let Ok(Fork::Child) = daemon(true, false) {
                    // 启动子进程
                    let mut cmd = process::Command::new(&args[0]);

                    for idx in 1..args.len() {
                        let arg = args.get(idx).expect("get cmd arg error!");
                        // 去除后台启动参数,避免重复启动
                        if arg.eq("-d") || arg.eq("-daemon") {
                            continue;
                        }
                        cmd.arg(arg);
                    }

                    let child = cmd.spawn().expect("Child process failed to start.");
                    fs::write("pid", child.id().to_string()).unwrap();
                    println!("process id is:{}", std::process::id());
                    println!("child id is:{}", child.id());
                }
                println!("{}", "daemon mod");
                process::exit(0);
            }
            start("by_fork:".to_string());
        }
        if let Some(startbydaemonize) = server.subcommand_matches("bydaemonize") {
            println!("start by daemonize");
            let base_dir = env::current_dir().unwrap();
            if startbydaemonize.get_flag("daemon") {
                let stdout = File::create("/tmp/daemon.out").unwrap();
                let stderr = File::create("/tmp/daemon.err").unwrap();

                println!("{:?}", base_dir);

                let daemonize = Daemonize::new()
                    .pid_file("/tmp/test.pid") // Every method except `new` and `start`
                    .chown_pid_file(true) // is optional, see `Daemonize` documentation
                    .working_directory(base_dir.as_path()) // for default behaviour.
                    // .user("nobody")
                    // .group("daemon") // Group name
                    .umask(0o777) // Set umask, `0o027` by default.
                    .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
                    .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
                    .privileged_action(|| "Executed before drop privileges");

                match daemonize.start() {
                    Ok(_) => {
                        println!("Success, daemonized");
                    }
                    Err(e) => eprintln!("Error, {}", e),
                }
            }
            println!("pid is:{}", std::process::id());
            // let mut path = base_dir.clone();
            // path.push("pid");
            // fs::write(path, process::id().to_string()).unwrap();
            fs::write("pid", process::id().to_string()).unwrap();
            start("by_daemonize:".to_string());
        }
    }
}