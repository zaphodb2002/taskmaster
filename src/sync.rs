use crate::Task;
use std::{fs::{File,  self, OpenOptions}, io::Write};
use anyhow::Result;
use task_hookrs::{task::TW26, import::import_task};
use walkdir::{WalkDir, DirEntry};
use serde_json;

const MD_FILE_ROOT :&str = "/home/zaphod/Documents/Test/";

pub struct TWSync {
    
}

impl TWSync {
    pub fn import(path :String) -> Result<Vec<Task>>{

        let dir_tree = get_dir_entries_recursively(&path)?;
        let mut tasks :Vec<Task> = Vec::new();
        
        for entry in &dir_tree {
            dbg!(entry);
            if entry.clone().into_path().is_file() {
                let task = read_from_md_file(entry.clone())?;
                tasks.push(task);
            }
        }

        for task in &tasks {
            if md_needs_update(&task) {
                 let md :File = write_md_file_for_task(&task)?;
            }
        }
        Ok(tasks)
    }

    pub fn export(path :&str) -> Result<Vec<Task>> {
        let dir_tree = get_dir_entries_recursively(MD_FILE_ROOT)?;
        let mut tasks :Vec<Task> = Vec::new();
        dir_tree.iter().for_each(|entry| {
            let task = read_from_md_file(entry.clone()).unwrap();
            if entry.path().is_file() {
               tasks.push(task);
            }
        });
        for task in &tasks {
            let json = &convert_to_json(task)?;
            let filename = path.to_string() + &task.uuid() + ".json";
            
            let mut file = File::create(filename)?;
            file.write_all(json.as_bytes())?;   
        }

        Ok(tasks)
    }
}

fn convert_to_toml(task :&Task) -> Result<String> {
    let result = toml::to_string(task)?;
    Ok(result)
}

fn convert_to_json(task :&Task) -> Result<String> {
    let result = serde_json::to_string(task)?;
    Ok(result)
}

fn md_needs_update(task :&Task) -> bool {
    true
}

fn read_from_md_file(entry: DirEntry) -> Result<Task> {
    let json = fs::read_to_string(entry.path())?;
    let tw26 = import_task::<TW26>(&json)?;
    let task = Task::new(tw26);
    Ok(task)
}

fn write_md_file_for_task(task :&Task) -> Result<File> {
    let result = create_or_open_md_file(task)?;
    Ok(result)
}

fn create_md_folder(task: &Task) -> Result<String> {
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);
    
    let mut path = MD_FILE_ROOT.to_string();
    let project_raw = task.project();
    let project = project_raw.split(".");
    for level in project {
        path += level;
        path += "/";
    }

    let _ = builder.create(path.clone());
    Ok(path)
}

fn create_or_open_md_file(task :&Task) -> Result<File> {
    let mut path = create_md_folder(task)?;
    let filename = task.uuid() + ".md";
    path += &filename;

    let file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)?;

    Ok(file)
}


fn get_dir_entries_recursively(path :&str) -> Result<Vec<DirEntry>> {
    let entries = WalkDir::new(&path).into_iter()
                        .map(|r| r.unwrap())
                        .filter(|entry| {
                        entry.path().is_file()
                    }).collect();
    Ok(entries)
}

