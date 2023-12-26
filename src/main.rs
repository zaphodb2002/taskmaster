
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

mod parser;

use std::fs;
use std::path::Path;
use std::env;
use std::path::PathBuf;

use clap::{arg, command, Command, value_parser};
use anyhow::{Result, Error};

fn main() {
    let matches = command!() // imports package info from Cargo.toml
        .subcommand(
            Command::new("import")
            .about("imports TaskWarrior JSON arrays")
            .arg(arg!(
                    <FILE> "path to taskwarrior export data"
                    )
                .required(true)
                .value_parser(value_parser!(PathBuf))
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("import", sub_matches)) => {
            cmd_import(sub_matches.get_one::<PathBuf>("FILE").expect("Path failed!"))
        },
        _ => unreachable!("WTF is this?")
    };

//      Command::Add => panic!("Command 'ADD' is not yet implemented."),
//      Command::Remove => panic!("Command 'REMOVE' is not yet implemented."),
//      Command::Modify => panic!("Command 'MODIFY' is not yet implemented."),
//      Command::Report => cmd_report(params),
//      Command::Export => panic!("Command 'EXPORT' is not yet implemented."),
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
    
    let report = match params[0] {
        "progress" => params[0],
        "project" => params[0],
        "gtd" => params[0],
        _ => "Report not Recognized"
    };

    if report == "progress" {
        
    }

    else if report == "project" {
        if params.len() > 1 {

            let project = match params[1] {
                "lms" => "LMS".to_string(),
                "subsistence" => "Subsistence".to_string(),
                _ => "Project not recognized".to_string(),
            };

            filtered_tasks.append(&mut get_tasks_by_project(project, &tasks));

            dbg!(filtered_tasks.len());
        }
        else {
            text = "No project specified".to_string();
        }
            
    }

    else if report == "gtd" {
        filtered_tasks.append(&mut get_tasks_for_gtd_report(&tasks));          
    }


    for task in filtered_tasks {
        text += &task.format_for_report();
        text += "\n";
    }

    let result = CommandResult {
        tasks: tasks,
        text: text
    };

    result
}

fn get_tasks_for_gtd_report(tasks :&Vec<Task>) -> Vec<Task> {
    let mut result :Vec<Task> = Vec::new();
    // TODO: Need a global list of all projects
    panic!("Not Yet Implemented");
    result
}

fn get_tasks_from_local() -> Vec<Task> {
    let jsons = get_all_jsons_recursive(LOCAL_PATH.into()).unwrap();
    let tasks = read_tasks_from_json(jsons).unwrap();
    tasks
}

fn get_tasks_by_project(project :String, tasks:&Vec<Task>) -> Vec<Task> {
    let mut result :Vec<Task> = Vec::new();
    for task in tasks {
        if task.project.contains(&project) {
            
            result.push(task.clone());
        }
    }

    dbg!(result.len());
    result
}


/// Command: Import
/// &str -> CommandResult
/// processes a given list of filepaths (recursively if directory) and imports any valid task jsons into our
/// local file structure, and returns a l
fn cmd_import(path :&PathBuf) -> CommandResult {
    let jsons = get_all_jsons_recursive(path.to_path_buf()).unwrap();
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

fn get_all_jsons_recursive(path :PathBuf) -> Result<Vec<String>,Error> {

    let mut jsons :Vec<String> = Vec::new();
    let mut new_jsons = load_jsons(&path, &mut jsons).unwrap();
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

fn load_jsons(pathbuf :&PathBuf, jsonpool :&mut Vec<String>) -> Result<Vec<String>,Error>{
    let path = Path::new(pathbuf);
    if path.is_dir() {
        for file in fs::read_dir(path).unwrap() {
            let file = file.unwrap();
            let path = &file.path();
            if path.is_file() {
                let json = fs::read_to_string(path).unwrap();
                jsonpool.push(json);
            }
            else{
                load_jsons(pathbuf, jsonpool);
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

