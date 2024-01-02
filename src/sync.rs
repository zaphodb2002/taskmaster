use std::{path::PathBuf, fs::{File}, io::BufReader};
use anyhow::Result;
use task_hookrs::{task::{Task, TW26}, import::import};
use walkdir::{WalkDir, DirEntry};

pub struct TWSync {

}

impl TWSync {
    pub fn import(path :PathBuf) -> Result<Vec<Task>>{
        let dir_tree = get_dir_entries_recursively(path)?;
        let tasks :Vec<Task> = Vec::new();
        
        for entry in &dir_tree {
            if entry.clone().into_path().is_file() {
                let _ = write_md_file_for_task(entry.clone());
            }
        }

        Ok(tasks)
    }
}

fn write_md_file_for_task(entry: DirEntry) -> Result<()> {
    let file = File::open(entry.path())?;
    let tasks = import::<TW26, File>(file);
    Ok(())
}

fn get_dir_entries_recursively(path :PathBuf) -> Result<Vec<DirEntry>> {
    let mut result :Vec<DirEntry> = Vec::new();
    
    let entries =WalkDir::new(path);
    
    for entry in entries {
        result.push(entry?);
    }

    Ok(result)
}

fn get_dirs(path :PathBuf) -> Result<Vec<DirEntry>> {
    let mut result :Vec<DirEntry> = Vec::new();
    let dirs = get_dir_entries_recursively(path);

    dbg!(&dirs);
    Ok(result)

}

//fn get_files(path :PathBuf) -> Result<Vec<DirEntry>> {
    
//    let files :Vec<_> = get_dir_entries_recursively(path).into_iter().filter(|e| {
//        e.as_ref().unwrap().path().is_file()
//    }).collect();

//}




