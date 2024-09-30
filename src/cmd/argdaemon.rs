use clap::{Arg, ArgAction, ArgMatches};
use fork::{daemon, Fork};
use std::process::Command;
use std::time::Duration;
use std::{env, fs, process, thread};

pub fn arg_init() -> Arg {
    Arg::new("daemon")
        .short('d')
        .long("daemon")
        .action(ArgAction::SetTrue)
        .help("run as daemon")
}

pub fn daemon_command(matches: &ArgMatches) {
    if matches.get_flag("daemon") {
        // if matches.is_present("daemon") {
        let args: Vec<String> = env::args().collect();
        if let Ok(Fork::Child) = daemon(true, true) {
            // 启动子进程
            let mut cmd = Command::new(&args[0]);

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
        thread::sleep(Duration::from_millis(20000));
        process::exit(0);
    }
}
