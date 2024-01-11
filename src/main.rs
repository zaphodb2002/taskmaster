mod sync;
mod report;
use crate::report::Report;

use sync::TWSync;

mod task;
use crate::task::Task;

use anyhow::Result;
use clap::{arg, command, Command};

const JSON_INBOX :&str = "/home/zaphod/.task/inbox/";
const JSON_OUTBOX :&str ="/home/zaphod/.task/outbox/";

fn main() {
    let matches = match_input();
    let result = process_cmd(matches);
   
/////////
// Output
    println!("{}",result.unwrap());
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
//              .arg(arg!(<REPORT> "Report format to output"))
        )
        .get_matches()

}

fn process_cmd(matches :clap::ArgMatches) -> Result<String>{
    match matches.subcommand() {
        Some(("import", _submatches)) => cmd_import(),
        Some(("export", _submatches)) => cmd_export(),
        Some(("report", _submatches)) => cmd_report(),
        _ => unreachable!("WTF is this?"),
    }
}

fn cmd_import() -> Result<String> {
    let tasks = TWSync::import(JSON_INBOX.into())?;
    let result = Report::import(tasks);
    Ok(result)
}

fn cmd_export() -> Result<String> {
    let tasks = TWSync::export(JSON_OUTBOX.into())?;
    let result = Report::export(tasks);
    Ok(result)
}

fn cmd_report() -> Result<String> {
    let report = Report::full()?;
    Ok(report.to_string())

}
