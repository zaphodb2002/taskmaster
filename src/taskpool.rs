use std::fs::{File, DirEntry};
use std::{fs, path::PathBuf};
use anyhow::{Result, Error};
use rayon::prelude::*;
use crate::parser;
use crate::task::Task;

#[derive(Clone)]
pub(crate) struct TaskPool {
    pub tasks :Vec<Task>
}

impl TaskPool {
    pub fn import(path: PathBuf) -> TaskPool {
        let jsons = get_all_jsons_recursive(path.to_path_buf()).unwrap();
        let tasks = read_tasks_from_json(jsons).unwrap();
        let result = write_tasks_to_files(&tasks);


        result
    }
    pub fn get_all_tasks() -> TaskPool {
            TaskPool {
                tasks: get_tasks_from_local()
            }
    }

    pub fn get_tasks_by_project(&self, project: String) -> TaskPool {
        let mut result :Vec<Task>= Vec::new();
        for task in &self.tasks {
            if task.project.contains(&project) {
                result.push(task.clone());
            }
        }

        TaskPool{
            tasks: result
        }                      
    }
}

fn load_jsons(pathbuf: &PathBuf, jsonpool: &mut Vec<String>) -> Result<Vec<String>, Error> {
    let path = pathbuf.as_path();
    if path.is_dir() {
        let files = fs::read_dir(&path);
        files?
            .map(|file| file.expect("file error").path())
            .for_each(
                |f| {
                    if f.is_dir() {
                        let _ = load_jsons(&f, jsonpool);
                    } else {
                        jsonpool.push(fs::read_to_string(f).expect(""));
                    }
                }
            );
    }
    Ok(jsonpool.to_vec())
}

fn get_all_jsons_recursive(path: PathBuf) -> Result<Vec<String>, Error> {
    let mut jsons: Vec<String> = Vec::new();
    let mut new_jsons = load_jsons(&path, &mut jsons).unwrap();
    jsons.append(&mut new_jsons);

    Ok(jsons)
}

const LOCAL_PATH :&str = "./pages";
fn get_tasks_from_local() -> Vec<Task> {
    let path :PathBuf = LOCAL_PATH.into();
    dbg!(path.parent());
    let jsons = get_all_jsons_recursive(LOCAL_PATH.into()).unwrap();
    dbg!(jsons.len());
    let tasks = read_tasks_from_json(jsons).unwrap();
    tasks
}

fn read_tasks_from_json(jsons: Vec<String>) -> Result<Vec<Task>, Error> {
    let mut result: Vec<Task> = Vec::new();
    for json in jsons {
        let task = parser::parse(&json);
        let task = task.unwrap();
        result.push(task);
    }
    Ok(result)
}

fn write_tasks_to_files(tasks: &Vec<Task>) -> TaskPool {
    let mut result = Vec::new();
    for task in tasks {
        task.write();
        result.push(task.clone());
        dbg!(&task.description);
    }
    TaskPool{
        tasks: result
    }
}


