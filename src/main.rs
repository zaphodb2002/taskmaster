
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

mod task;
use command::CommandResult;
use task::Task;

mod command;
use command::Command;

mod parser;

use std::io;
use std::fs;
use std::path::Path;
use std::env;




//const TASK_COMPLETION_THRESHOLD_HIGH :f64 = 0.9;
//const TASK_COMPLETION_THRESHOLD_LOW :f64 = 0.5;

fn main() -> io::Result<()>{

    let args :Vec<String> = env::args().collect();
    let mut temp_cmd = "help";
    if args.len() > 1 {
        temp_cmd = &args[1];
    }
    let command = Command::from(temp_cmd.to_string());
    
    let mut params :Vec<&str> = Vec::new();
    for i in 2..args.len() {
        params.push(&args[i]);
    }
    let jsonpool :Vec<String> = Vec::new();
    let taskpool :Vec<Task> = Vec::new();

    let result = match command {
        Command::Add => panic!("Command 'ADD' is not yet implemented."),
        Command::Remove => panic!("Command 'REMOVE' is not yet implemented."),
        Command::Modify => panic!("Command 'MODIFY' is not yet implemented."),
        Command::Report => cmd_report(params),
        Command::Import => cmd_import(params),
        Command::Export => panic!("Command 'EXPORT' is not yet implemented."),
        Command::Help => cmd_help(params),
        Command::Version=> cmd_version(params),
        Command::NotRecognized => cmd_not_recognized(params),
    };
    println!("{}",result.text);
    Ok(())
}

const TEST_DATA_PATH_STR :&str = "./test_data/";
const LOCAL_PATH :&str = "./pages/";

/// Command: Report
/// &str -> CommandResult
/// reads tasks from the local file structure and outputs a well-formatted report
fn cmd_report(params :Vec<&str>) -> CommandResult {
    let tasks = get_tasks_from_local();
    dbg!(tasks.len());
    let mut text = String::new();
    
    let mut filtered_tasks :Vec<Task>= Vec::new();
    
    for task in tasks.clone() {
       if task.project.contains(&"Subsistence".to_string()){
           if task.end == None {
                filtered_tasks.push(task);
           }
       }
    }

    dbg!(filtered_tasks.len());

    let result = CommandResult {
        tasks: tasks,
        text
    };

    result
}

fn get_tasks_from_local() -> Vec<Task> {
    let jsons = get_all_jsons_recursive(LOCAL_PATH).unwrap();
    let tasks = read_tasks_from_json(jsons).unwrap();
    tasks
}

/// Command: Import
/// &str -> CommandResult
/// processes a given list of filepaths (recursively if directory) and imports any valid task jsons into our
/// local file structure, and returns a l
fn cmd_import(path :Vec<&str>) -> CommandResult {
    let jsons = get_all_jsons_recursive(path[0]).unwrap();
    let tasks = read_tasks_from_json(jsons).unwrap();

    dbg!(tasks.len());
    let saved_tasks = match write_tasks_to_files(&tasks){
        Ok(tasks) => tasks,
        Err(e) => panic!("{}",e)
    };

    let result = CommandResult {
        tasks: saved_tasks,
        text: "Tasks imported".to_string(),
    };

    result
}

fn get_all_jsons_recursive(path :&str) -> Result<Vec<String>,&str> {

    let mut jsons :Vec<String> = Vec::new();
    let mut new_jsons = load_jsons(path, &mut jsons).unwrap();
    jsons.append(&mut new_jsons);
    
    Ok(jsons)
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
// TODO: This should be recursive

// 

fn load_jsons(path :&str, jsonpool :&mut Vec<String>) -> Result<Vec<String>,&'static str>{
    let path = Path::new(path);
    if path.is_dir() {
        for file in fs::read_dir(path).unwrap() {
            let file = file.unwrap();
            let path = &file.path();
            //dbg!(path);
            if path.is_file() {
                let json = fs::read_to_string(path).unwrap();
                jsonpool.push(json);
                //dbg!(jsonpool.len());
            }
            else{
                load_jsons(path.to_str().unwrap(), jsonpool);
            }
        }
    }
    Ok(jsonpool.to_vec())
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

fn cmd_version(_param :Vec<&str>) -> CommandResult {
    let result = CommandResult{
        tasks: vec![],
        text: VERSION_STR.to_string() 
    };
    result
}
// Command: Not Recognized
const NOT_RECOGNIZED_STR :&str = "Command not recognized.";

fn cmd_not_recognized(_param :Vec<&str>) -> CommandResult {
    let result = CommandResult{
        tasks: vec![],
        text: NOT_RECOGNIZED_STR.to_string() 
    };
    result
}

// Command: Help
const HELP_STR :&str = "This is the help file.";

fn cmd_help(_params :Vec<&str>) -> CommandResult {
    let result = CommandResult{
        tasks: vec![],
        text: HELP_STR.to_string() 
    };
    result
}

