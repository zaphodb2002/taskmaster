
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
use std::env;
use crate::{taskpool::TaskPool, task::Task};


mod parser;
mod task;
mod taskpool;


const TASK_COMPLETION_THRESHOLD_HIGH :f64 = 0.9;
const TASK_COMPLETION_THRESHOLD_LOW :f64 = 0.5;

fn main() -> io::Result<()>{
    let vec = vec![];
    let mut taskpool :TaskPool= TaskPool(vec);


    run_tests();

    let args :Vec<String> = env::args().collect();
    let tmp_cmd = args[1].to_lowercase();
    let command = tmp_cmd.as_str();
    
    let mut params :Vec<&str> = Vec::new();
    for i in 2..args.len() {
        params.push(&args[i]);
    }
       
    let result = match command {
//        "add" => println!("Command 'ADD' is not yet implemented."),
//        "remove" => println!("Command 'REMOVE' is not yet implemented."),
//        "modify" => println!("Command 'MODIFY' is not yet implemented."),
//        "report" => println!("Command 'REPORT' is not yet implemented."),
        "import" => cmd_import(params),
//        "export" => println!("Command 'EXPORT' is not yet implemented."),
//        "help" => println!("Command 'HELP' is not yet implemented."),
        "version" => cmd_version(params),
        _ => cmd_not_recognized(params),
    };
    
    dbg!(result);

    //let file = "/home/zaphod/export.data";
    //let raw = fs::read_to_string(file)?;
    //let split = raw.split('\n');
    //let mut tasks :Vec<Task> = Vec::new();
    //for line in split {
    //    let opt = parser::parse(line.to_string());
    //    if opt.is_some(){
    //        tasks.push(opt.unwrap());
    //    }
    //}

    Ok(())
}

/// Command: Import
/// &str -> Result<Vec<&str>>
/// processes a given list of filepaths (recursively if directory) and imports any valid task jsons into our
/// local data structure.  returns a Vec of task uuids that were successfully imported into the
/// data structure.

const TEST_DATA_PATH_STR :&str = "./test_data/";
fn tests_cmd_import(){
    let correct_vec :Vec<&str> = Vec::new();
    assert_eq!(cmd_import([TEST_DATA_PATH_STR].to_vec()), Ok(correct_vec));
}

fn cmd_import(params :Vec<&str>) -> Result<Vec<&str>,&str> {
    let mut result :Vec<&str> = Vec::new();
    for param in params {
        let parsed :Option<Task> = parser::parse(param);

    };

    Ok(result)
}

// Command: Version
const VERSION_STR :&str = "TaskMaster v0.0.0alpha";

fn tests_cmd_version(){
    let correct_vec :Vec<&str> = vec![VERSION_STR];
    assert_eq!(cmd_version([TEST_DATA_PATH_STR].to_vec()), Ok(correct_vec));
}

fn cmd_version(param :Vec<&str>) -> Result<Vec<&str>,&str> {
    let result :Vec<&str> = vec![VERSION_STR]; 
    Ok(result)
}
// Command: Not Recognized
const NOT_RECOGNIZED_STR :&str = "Command not recognized.";
fn tests_cmd_not_recognized(){
    let correct_vec :Vec<&str> = vec![NOT_RECOGNIZED_STR];
    assert_eq!(cmd_not_recognized([TEST_DATA_PATH_STR].to_vec()), Ok(correct_vec));
}

fn cmd_not_recognized(param :Vec<&str>) -> Result<Vec<&str>,&str> {
    let result :Vec<&str> = vec![NOT_RECOGNIZED_STR]; 
    Ok(result)
}

fn run_tests(){
    tests_cmd_import();
    tests_cmd_version();
    tests_cmd_not_recognized();
}
