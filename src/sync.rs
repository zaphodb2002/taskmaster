use crate::Task;
use anyhow::Result;
use std::{
    ffi::OsStr,
    path::PathBuf,
};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};
use task_hookrs::{import::import_task, task::TW26};
use walkdir::{DirEntry, WalkDir};

pub mod local_data;
pub mod tw_sync;
use local_data::LocalData;

pub trait Sync {
    fn sync(&self) -> Result<()>;
}

pub trait Import {
    fn import(&self, local_data: LocalData) -> Result<Vec<Task>>;
}

pub trait Export {
    fn export(&self) -> Result<Vec<Task>>;
}

fn get_all_tasks_at(path: &PathBuf) -> Vec<Task> {
    let dir_tree = get_dir_entries_recursively(path).expect("bad dir_tree");
    let files = get_files_from_dir_tree(dir_tree);
    let mut tasks: Vec<Task> = Vec::new();
    for file in files {
        let filepath = file.path().to_path_buf();
        let task = read_from_file(filepath);

        if task.is_some() {
            tasks.push(task.unwrap());
        }
    }

    tasks
}

fn get_dir_entries_recursively(path: &PathBuf) -> Result<Vec<DirEntry>> {
    dbg!(&path);
    let entries = WalkDir::new(&path)
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .filter(|entry| entry.path().is_file())
        .collect();
    Ok(entries)
}

fn get_files_from_dir_tree(dir_tree: Vec<DirEntry>) -> Vec<DirEntry> {
    dir_tree
        .into_iter()
        .filter(|entry| {
            entry.path().is_file() && entry.metadata().expect("bad metadata?").len() > 0
        })
        .collect()
}

fn read_from_file(path: PathBuf) -> Option<Task> {
    let extension = path.extension();
    if extension == Some(OsStr::new("json")) {
        return Some(read_from_json(path).unwrap());
    } else if extension == Some(OsStr::new("toml")) {
        return Some(read_from_toml(path).unwrap());
    } else {
        None
    }
}

fn read_from_md(path: PathBuf) -> Result<Task> {
    todo!()
}

fn read_from_toml(path: PathBuf) -> Result<Task> {
    Ok(local_data::read_from_toml_file(path)?)
}

fn read_from_json(path: PathBuf) -> Result<Task> {
    tw_sync::read_from_json_file(path)
}
