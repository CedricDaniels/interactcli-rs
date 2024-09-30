use crate::request::{ReqResult, Request, RequestTaskListAll};
use clap::{arg, ArgMatches, Command};
use std::fs::File;
use std::io::Read;

pub fn cmd_init() -> Command {
    clap::Command::new("task")
        .about("command about task")
        .subcommand(cmd_task_create())
        .subcommand(cmd_task_start())
        .subcommand(cmd_task_stop())
        .subcommand(cmd_task_remove())
        .subcommand(cmd_task_list())
}

fn cmd_task_create() -> Command {
    clap::Command::new("create")
        .about("create task")
        .arg(arg!(<path> "create task json file path"))
}

fn cmd_task_start() -> Command {
    clap::Command::new("start")
        .about("start task")
        .arg(arg!(<taskid> "input task id to stop"))
}

fn cmd_task_stop() -> Command {
    clap::Command::new("stop")
        .about("stop task")
        .arg(arg!(<taskid>  "input task id to stop"))
}

fn cmd_task_remove() -> Command {
    clap::Command::new("remove")
        .about("remove task")
        .arg(arg!(<taskid>  "input task id to stop"))
}

fn cmd_task_list() -> Command {
    clap::Command::new("list")
        .about("list tasks")
        .subcommand(cmd_task_list_all())
        .subcommand(cmd_task_list_by_ids())
        .subcommand(cmd_task_list_by_names())
}

fn cmd_task_list_all() -> Command {
    clap::Command::new("all")
        .about("list tasks by task ids")
        .arg(arg!([queryid] "input queryid if have"))
}

fn cmd_task_list_by_ids() -> Command {
    clap::Command::new("byid")
        .about("list tasks by task ids")
        .arg(arg!(<taskid> "input taskid"))
}

fn cmd_task_list_by_names() -> Command {
    clap::Command::new("bynames")
        .about("list tasks by task names")
        .arg(arg!(<tasksname>
            r"input tasks name if multi use ',' to splite"
        ))
}

pub fn cmd_exec(req: Request, matches: &ArgMatches) {
    if let Some(ref matches) = matches.subcommand_matches("task") {
        if let Some(create) = matches.subcommand_matches("create") {
            let file = File::open(create.get_one::<String>("path").unwrap());
            // let file = File::open(create.value_of("path").unwrap());
            match file {
                Ok(mut f) => {
                    let mut data = String::new();
                    if let Err(e) = f.read_to_string(&mut data) {
                        println!("{}", e);
                        return;
                    };
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        let resp = req.create_task(data).await;
                        let result = ReqResult::new(resp);
                        result.normal_parsor().await;
                    };
                    rt.block_on(async_req);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        if let Some(start) = matches.subcommand_matches("start") {
            if let Some(taskid) = start.get_one::<String>("taskid") {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let async_req = async {
                    let resp = req.task_start(taskid.to_string()).await;
                    let result = ReqResult::new(resp);
                    result.normal_parsor().await;
                };
                rt.block_on(async_req);
            };
        }
        if let Some(stop) = matches.subcommand_matches("stop") {
            if let Some(taskid) = stop.get_one::<String>("taskid") {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let async_req = async {
                    let resp = req.task_stop(taskid.to_string()).await;
                    let result = ReqResult::new(resp);
                    result.normal_parsor().await;
                };
                rt.block_on(async_req);
            };
        }
        if let Some(remove) = matches.subcommand_matches("remove") {
            if let Some(taskid) = remove.get_one::<String>("taskid") {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let async_req = async {
                    let resp = req.task_stop(taskid.to_string()).await;
                    let result = ReqResult::new(resp);
                    result.normal_parsor().await;
                };
                rt.block_on(async_req);
            };
        }
        if let Some(list) = matches.subcommand_matches("list") {
            match list.subcommand_name() {
                Some("all") => {
                    let queryid = list
                        .subcommand_matches("all")
                        .unwrap()
                        .get_one::<String>("queryid");
                    let mut module = RequestTaskListAll::default();
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        match queryid {
                            None => {
                                let resp = req.task_list_all(module).await;
                                let result = ReqResult::new(resp);
                                result.task_list_all_parsor().await;
                            }
                            Some(id) => {
                                module.set_query_id(id.to_string());
                                let resp = req.task_list_all(module).await;
                                let result = ReqResult::new(resp);
                                result.task_list_all_parsor().await;
                            }
                        }
                    };
                    rt.block_on(async_req);
                }
                Some("byid") => {
                    let queryid = list
                        .subcommand_matches("byid")
                        .unwrap()
                        .get_one::<String>("taskid");
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        let mut ids = vec![];
                        if let Some(id) = queryid {
                            ids.push(id.to_string());
                            let resp = req.task_list_by_ids(ids).await;
                            let result = ReqResult::new(resp);
                            result.task_list_byid_parsor().await;
                        }
                    };
                    rt.block_on(async_req);
                }
                Some("bynames") => {
                    let names = list
                        .subcommand_matches("bynames")
                        .unwrap()
                        .get_one::<String>("tasksname");
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        // let mut namearry = names;
                        if let Some(namesstr) = names {
                            let namearry = namesstr.split(',').collect::<Vec<&str>>();

                            let resp = req.task_list_by_names(namearry).await;
                            let result = ReqResult::new(resp);
                            result.task_list_bynames_parsor().await;
                        }
                    };
                    rt.block_on(async_req);
                }

                _ => {}
            }
        }
    }
}
