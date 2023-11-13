
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
use std::path::Path;
use std::env;
use crate::task::Task;


mod parser;
mod task;
mod command;

//const TASK_COMPLETION_THRESHOLD_HIGH :f64 = 0.9;
//const TASK_COMPLETION_THRESHOLD_LOW :f64 = 0.5;

fn main() -> io::Result<()>{
    let taskpool :Vec<Task> = Vec::new();

    run_tests(&taskpool);

    let args :Vec<String> = env::args().collect();
    let mut temp_cmd = "help";
    if args.len() > 1 {
        temp_cmd = &args[1];
    }
    let command = temp_cmd;
    
    let mut params :Vec<&str> = Vec::new();
    for i in 2..args.len() {
        params.push(&args[i]);
    }
       
    let _result = match command {
//        "add" => println!("Command 'ADD' is not yet implemented."),
//        "remove" => println!("Command 'REMOVE' is not yet implemented."),
//        "modify" => println!("Command 'MODIFY' is not yet implemented."),
//        "report" => println!("Command 'REPORT' is not yet implemented."),
        "import" => cmd_import(vec![TEST_DATA_PATH_STR], taskpool),
//        "export" => println!("Command 'EXPORT' is not yet implemented."),
        //"help" => cmd_help(params),
        //"version" => cmd_version(params),
        _ => panic!(),//cmd_not_recognized(params),
    };
    Ok(())
}

const TEST_DATA_PATH_STR :&str = "./test_data/";

/// Command: Import
/// &str -> Result<Vec<Task>>
/// processes a given list of filepaths (recursively if directory) and imports any valid task jsons into our
/// local data structure, and returns said data structure
fn tests_cmd_import(_taskpool :&Vec<Task>){
    let _correct_vec :Vec<Task> = Vec::new();
   // assert_eq!(cmd_import([TEST_DATA_PATH_STR].to_vec(), taskpool).unwrap(), correct_vec);
}

fn cmd_import(paths :Vec<&str>, mut taskpool :Vec<Task>) -> Result<Vec<Task>, &str> {
    for path in paths {
        let jsons = load_jsons(path);
        let jsons = jsons.unwrap();
        let tasks = read_tasks_from_json(jsons);
        for task in tasks.unwrap() {
            taskpool.push(task);
        }
    }
    write_tasks_to_files(&taskpool);
    Ok(taskpool)
}

fn write_tasks_to_files(tasks :&Vec<Task>) -> Result<Vec<Task>,&'static str>{
    let result = Vec::new();
    for task in tasks {
        task.write();
    }
    Ok(result)
}

/// &str -> Result<Vec<&str>>
/// interp path as path to json files, outputs Vec of json strings
fn load_jsons(path :&str) -> Result<Vec<String>,&str>{
    let fixed_path = Path::new(path);
    let mut result :Vec<String>= Vec::new();
    if fixed_path.is_dir() {
        for file in fs::read_dir(fixed_path).unwrap() {
            let file = file.unwrap();
            let path = file.path();
            if path.is_dir() {
                for file in fs::read_dir(path).unwrap() {
                    let file = file.unwrap();
                    let path = file.path();
                    if path.is_file() {
                        let json = fs::read_to_string(path);
                        let json = json.unwrap();
                        //dbg!(&json);
                        result.push(json);
                    }
                }
            }
        }
    }
    Ok(result)
}

fn read_tasks_from_json<'a>(jsons :Vec<String>) -> Result<Vec<Task>, &'a str> {
    let mut result :Vec<Task> = Vec::new();
    for json in jsons {
        let task = parser::parse(&json);
        let task = task.unwrap();
        result.push(task);
    }
    Ok(result)
}

// Command: Version
const VERSION_STR :&str = "TaskMaster v0.0.0alpha";

fn tests_cmd_version(){
    let correct_vec :Vec<&str> = vec![VERSION_STR];
    assert_eq!(cmd_version([TEST_DATA_PATH_STR].to_vec()), Ok(correct_vec));
}

fn cmd_version(_param :Vec<&str>) -> Result<Vec<&str>,&str> {
    let result :Vec<&str> = vec![VERSION_STR]; 
    Ok(result)
}
// Command: Not Recognized
const NOT_RECOGNIZED_STR :&str = "Command not recognized.";
fn tests_cmd_not_recognized(){
    let correct_vec :Vec<&str> = vec![NOT_RECOGNIZED_STR];
    assert_eq!(cmd_not_recognized([TEST_DATA_PATH_STR].to_vec()), Ok(correct_vec));
}

fn cmd_not_recognized(_param :Vec<&str>) -> Result<Vec<&str>,&str> {
    let result :Vec<&str> = vec![NOT_RECOGNIZED_STR]; 
    Ok(result)
}

// Command: Help
const HELP_STR :&str = "This is the help file.";
fn tests_cmd_help(){
    let correct_vec :Vec<&str> = vec![HELP_STR];
    assert_eq!(cmd_help([TEST_DATA_PATH_STR].to_vec()), Ok(correct_vec));
}

fn cmd_help(_params :Vec<&str>) -> Result<Vec<&str>,&str> {
    let result :Vec<&str> = vec![HELP_STR];
    Ok(result)
}

fn run_tests(taskpool :&Vec<Task>){
    tests_cmd_import(&taskpool);
    tests_cmd_version();
    tests_cmd_not_recognized();
    tests_cmd_help();
}
