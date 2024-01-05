mod sync;
use std::path::Path;

use sync::TWSync;

mod task;
use crate::task::Task;

use anyhow::Result;
use clap::{arg, command, Command};

const JSON_INBOX :&str = "/home/zaphod/.task/inbox/";
const JSON_OUTBOX :&str ="/home/zaphod/.task/outbox/";

fn main() {
    let matches = match_input();
    process_cmd(matches);
   
/////////
// Output
    //for line in cmd_result.text {
    //    println!("{}", line);
    //}
}

fn match_input() -> clap::ArgMatches {
     command!() // imports package info from Cargo.toml
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
        .get_matches()

}

fn process_cmd(matches :clap::ArgMatches){
    let _ = match matches.subcommand() {
        Some(("import", _submatches)) => cmd_import(),
        Some(("export", _submatches)) => cmd_export(),
        _ => unreachable!("WTF is this?"),
    };
}

fn cmd_import() -> Result<Vec<Task>> {
    let path :String = JSON_INBOX.to_string();
    let tasks = TWSync::import(path)?;
    Ok(tasks)
}

fn cmd_export() -> Result<Vec<Task>> {
    let tasks = TWSync::export(JSON_OUTBOX.into())?;
    Ok(tasks)
}
