mod templates;

use super::*;
use anyhow::Result;

pub const MD_FILE_ROOT: &str = "/home/zaphod/Documents/Test/";

pub struct LocalData {
    pub tasks: Vec<Task>,
}

impl LocalData {
    pub fn new(path: PathBuf) -> Result<LocalData> {
        let tasks = get_all_tasks_at(&path);
        let result = LocalData { tasks };
        Ok(result)
    }

    pub fn add_task(&self, task: &Task) -> Result<()> {
        let _ = write_md_file_for_task(task)?;

        Ok(())
    }

    pub fn update_task(&self, task: &Task) -> Result<Task> {
        let new_file = write_toml_file_for_task(&task)?;
        let task = read_from_toml_file(new_file)?;
        Ok(task)
    }
}

pub fn read_from_toml_file(path: PathBuf) -> Result<Task> {
    let string = fs::read_to_string(path)?;
    let task: Task = toml::from_str(&string)?;
    Ok(task)
}

fn create_md_folder(task: &Task) -> Result<String> {
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);

    let mut path = MD_FILE_ROOT.to_string();
    let project_raw = task.project().to_string();
    let project = project_raw.split(".");
    for level in project {
        path += level;
        path += "/";
    }

    let _ = builder.create(path.clone());
    Ok(path)
}

fn write_md_file_for_task(task: &Task) -> Result<File> {
    let result = create_or_open_md_file(task)?;
    Ok(result)
}

fn create_or_open_md_file(task: &Task) -> Result<File> {
    let mut path = create_md_folder(task)?;
    let filename = format!("{}.md", task.uuid());
    path += &filename;

    let mut file = OpenOptions::new().write(true).create(true).open(path)?;

    let content = task.to_md();
    let _ = file.write_all(&content?.into_bytes())?;
    Ok(file)
}

fn write_json_file_for_task(task: &Task) -> Result<File> {
    let result = create_or_open_json_file(task)?;
    Ok(result)
}

fn write_toml_file_for_task(task: &Task) -> Result<PathBuf> {
    let result = create_or_open_toml_file(task)?;
    Ok(result)
}

fn create_or_open_json_file(task: &Task) -> Result<File> {
    let mut path = create_md_folder(task)?;
    let filename = format!("{}.json", task.uuid());
    path += &filename;

    let mut file = OpenOptions::new().write(true).create(true).open(path)?;

    let content = task.to_json();
    let _ = file.write_all(&content?.into_bytes())?;
    Ok(file)
}

fn create_or_open_toml_file(task: &Task) -> Result<PathBuf> {
    let mut path = create_md_folder(task)?;
    let filename = task.uuid() + ".toml";
    path += &filename;

    let mut file = OpenOptions::new().write(true).create(true).open(&path)?;

    let content = task.to_toml();
    let _ = file.write_all(&content?.into_bytes())?;
    Ok(path.into())
}
