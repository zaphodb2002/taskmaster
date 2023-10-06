
/////////////////////////
/// TaskMaster
///////////////////
///
/// TaskMaster is a tool to manage TaskWarrior and conform it to an opinionated workflow.///
/// Features
/// ========
/// - Consume TaskWarrior data and produce high quality reports
/// - Enforce conformity and improve usability of tasks and workflows via rules
/// - Enhance and/or replace TaskWarrior's handling of recurring tasks
/// - Create a file structure rather than a single flat file with 2 way interoperability
///
/// Consuming the Data
/// ==================
/// TaskWarrior can export JSON.
/// We read the JSON and store it as a Task struct
///
/// Reporting
/// =========
/// - Recurring task completion percent
///     - Lowest performing
///     - Highest performing
///     - by project and individually
///     - by timespan, for comparison
/// - Scheduling Assistance
///     - recurring tasks by day of week and time of day
///     - visual calendar layout
/// Rules
/// =====
/// - Enforce dependencies on recurring tasks
/// - Automatically fail a recurring task like the until date is supposed to
/// - Automatic adjustment of tags and dates based on each other
/// - Use TaskMaster's recurrence system instead for more flexibility (chron-like?)

use std::io;
use std::fs;
use crate::parser::Task;

pub mod parser;

const TASK_COMPLETION_THRESHOLD_HIGH :f64 = 0.9;
const TASK_COMPLETION_THRESHOLD_LOW :f64 = 0.5;



fn main() -> io::Result<()>{

    let file = "/home/zaphod/export.data";
    let raw = fs::read_to_string(file)?;
    let split = raw.split('\n');
    let mut tasks :Vec<Task> = Vec::new();
    for line in split {
        let opt = parser::parse(line.to_string());
        if opt.is_some(){
            tasks.push(opt.unwrap());
        }
    }

    dbg!(&tasks[0]);
    Ok(())
}



