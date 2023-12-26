mod task;
mod taskpool;
mod report;
use report::Report;
use taskpool::TaskPool;
use command::CommandResult;
use task::Task;

mod command;

mod parser;

use std::{env, path::PathBuf};

use anyhow::{Error, Result};
use clap::{arg, command, value_parser, Command};

fn main() {
////////
// Input
    let matches = command!() // imports package info from Cargo.toml
        .subcommand(
            Command::new("import")
                .about("imports TaskWarrior JSON arrays")
                .arg(
                    arg!(
                    <FILE> "path to taskwarrior export data"
                    )
                    .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("report")
                .about("outputs a report of tasks")
                .arg(arg!(<REPORT> "Report format to output"))
        )
        .get_matches();
///////////
// Process
    let cmd_result = match matches.subcommand() {
        Some(("import", sub_matches)) => cmd_import(
            sub_matches
                .get_one::<PathBuf>("FILE")
                .expect("Path failed!"),
        ),
        Some(("report", sub_matches)) => cmd_report(
            sub_matches
                .get_one::<String>("REPORT")
                .expect("This is required so it should work?")
            ),
        _ => unreachable!("WTF is this?"),
    };
/////////
// Output
    for line in cmd_result.text {
        println!("{}", line);
    }
    //      Command::Add => panic!("Command 'ADD' is not yet implemented."),
    //      Command::Remove => panic!("Command 'REMOVE' is not yet implemented."),
    //      Command::Modify => panic!("Command 'MODIFY' is not yet implemented."),
    //      Command::Report => cmd_report(params),
    //      Command::Export => panic!("Command 'EXPORT' is not yet implemented."),
}

fn cmd_report(report_type:&str) -> CommandResult {
    let report = match report_type {
        "gtd" => Report::gtd(),
        _ => panic!("bad report name")
    };
    let result = CommandResult {
        tasks: report.taskpool.clone(),
        text: report.format()
    };

    result
}

fn cmd_import(path: &PathBuf) -> CommandResult {

    let result = CommandResult {
        tasks: TaskPool::import(path.to_path_buf()),
        text: vec!("Tasks imported".to_string()),
    };

    result
}
