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
        
        dir_tree.into_iter().for_each(|entry| {
            let path = &entry.path();
            let name = path.file_name().expect("no name?");
            if path.is_file() {
                if path.metadata().expect("bad metadata?").len() > 0 {
                    match name.to_str().expect("conversion to str failed?") {
                        "null" => {println!("file at {} is null, need to not create these", path.display())}
                        _ => {
                            let task = read_from_json_file(&entry).expect(&format!("bad file: {}", &path.display()));
                            tasks.push(task);
                        }
                    }
                }
            }
        });
       
        for task in &tasks {

            if md_needs_update(&task) {
                 let _md :File = write_md_file_for_task(&task)?;
            }
            if json_needs_update(&task) {
                let _json :File = write_json_file_for_task(&task)?;
            }
            if toml_needs_update(&task) {
                let _toml :File = write_toml_file_for_task(&task)?;
            }
        }
        Ok(tasks)
    }


    pub fn export(path :&str) -> Result<Vec<Task>> {
        let dir_tree = get_dir_entries_recursively(MD_FILE_ROOT)?;
        let mut tasks :Vec<Task> = Vec::new();
        dir_tree.iter().for_each(|entry| {
            let path = &entry.path();
            let name = path.file_name().expect("no name?");
            if path.is_file(){
                if path.metadata().expect("bad metadata?").len() > 0 {
                    let task = read_from_md_file(entry.clone()).unwrap();
                        if entry.path().is_file() {
                    tasks.push(task);
                    }
                }
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
    let result = task.to_toml()?;
    Ok(result)
}

fn convert_to_json(task :&Task) -> Result<String> {
    let result = task.to_json()?;
    Ok(result)
}

fn md_needs_update(task :&Task) -> bool {
    true
}

fn json_needs_update(task :&Task) -> bool {
    true
}

fn toml_needs_update(task :&Task) -> bool {
    true
}
fn read_from_md_file(entry: DirEntry) -> Result<Task> {
    let md = fs::read_to_string(entry.path())?;
    let tw26 = import_task::<TW26>(&md)?;
    let task = Task::new(tw26);
    Ok(task)
}

fn read_from_json_file(entry: &DirEntry) -> Result<Task> {
    let json = fs::read_to_string(entry.path())?;
    let tw26 = import_task::<TW26>(&json)?;
    let task = Task::new(tw26);
    Ok(task)
}

fn read_from_toml_file(entry: DirEntry) -> Result<Task> {
    let toml = fs::read_to_string(entry.path())?;
    let tw26 = import_task::<TW26>(&toml)?;
    let task = Task::new(tw26);
    Ok(task)
}

fn write_md_file_for_task(task :&Task) -> Result<File> {
    let result = create_or_open_md_file(task)?;
    Ok(result)
}

fn write_json_file_for_task(task :&Task) -> Result<File> {
    let result = create_or_open_json_file(task)?;
    Ok(result)
}

fn write_toml_file_for_task(task :&Task) -> Result<File> {
    let result = create_or_open_toml_file(task)?;
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
fn create_or_open_json_file(task :&Task) -> Result<File> {
    let mut path = create_md_folder(task)?;
    let filename = task.uuid() + ".json";
    path += &filename;

    let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)?;
    
    let content = task.to_json();
    let _ = file.write_all(&content?.into_bytes())?;
    Ok(file)
}


fn create_or_open_toml_file(task :&Task) -> Result<File> {
    let mut path = create_md_folder(task)?;
    let filename = task.uuid() + ".toml";
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

