mod sync;
use crate::sync::{
    local_data::{LocalData, MD_FILE_ROOT},
    Export, Import,
};

mod task;
use crate::task::Task;

mod report;
use crate::report::Report;

use anyhow::Result;
use clap::{arg, command, Command};
use sync::tw_sync::TWSync;

const TW_PATH: &str = "/home/zaphod/.task";

fn main() {
    let matches = match_input();
    let result = process_cmd(matches);

    /////////
    // Output
    println!("{}", result.unwrap());
}

fn match_input() -> clap::ArgMatches {
    command!() // imports package info from Cargo.toml
        .subcommand(Command::new("import").about("imports TaskWarrior JSON arrays"))
        .subcommand(Command::new("export").about("exports TaskWarrior JSON arrays"))
        .subcommand(
            Command::new("report")
                .about("outputs a report of tasks")
                .arg(arg!(<REPORT> "Report format to output")),
        )
        .get_matches()
}

fn process_cmd(matches: clap::ArgMatches) -> Result<String> {
    match matches.subcommand() {
        Some(("import", _submatches)) => cmd_import(),
        Some(("export", _submatches)) => cmd_export(),
        Some(("report", _submatches)) => cmd_report(),
        _ => unreachable!("WTF is this?"),
    }
}

fn cmd_import() -> Result<String> {
    let local_data = LocalData::new(MD_FILE_ROOT.into())?;
    let tasks = TWSync::new(TW_PATH.into()).import(local_data)?;
    let result = Report::import(tasks);
    Ok(result)
}

fn cmd_export() -> Result<String> {
    let _local_data = LocalData::new(MD_FILE_ROOT.into());
    let tasks = TWSync::new(TW_PATH.into()).export()?;
    let result = Report::export(tasks);
    Ok(result)
}

fn cmd_report() -> Result<String> {
    let report = Report::gtd()?;
    Ok(report.display())
}
