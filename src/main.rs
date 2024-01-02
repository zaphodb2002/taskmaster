mod sync;
mod taskpool;
use sync::TWSync;
use anyhow::{Result};
use clap::{arg, command, value_parser, Command};
use task_hookrs::{import::import, task::{Task, TW26}};

const MD_FILE_ROOT :&str = "/home/zaphod/Documents/Test/";
const JSON_INBOX :&str = "/home/zaphod/.task/inbox/";
const JSON_OUTBOX :&str ="/home/zaphod/.task/outbox/";

fn main() {
////////
// Input
    let matches = command!() // imports package info from Cargo.toml
        .subcommand(
            Command::new("import")
                .about("imports TaskWarrior JSON arrays"),
        )
        .subcommand(
            Command::new("export")
                .about("exports TaskWarrior JSON arrays")
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
        Some(("import", sub_matches)) => cmd_import(),
        _ => unreachable!("WTF is this?"),
    };
/////////
// Output
    //for line in cmd_result.text {
    //    println!("{}", line);
    //}
}

fn cmd_import() -> Result<Vec<Task>> {
    let tasks = TWSync::import(JSON_INBOX.into())?;
    Ok(tasks)
}
