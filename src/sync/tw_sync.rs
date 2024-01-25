use crate::task::Task;
use anyhow::Result;

use super::*;

pub struct TWSync {
    outbox: PathBuf,
    inbox: PathBuf,
}
impl TWSync {
    pub fn new(path: PathBuf) -> TWSync {
        TWSync {
            outbox: format!("{}/outbox/", path.display()).into(),
            inbox: format!("{}/inbox/", path.display()).into(),
        }
    }
}

impl Import for TWSync {
    fn import(&self, local_data: LocalData) -> Result<Vec<Task>> {
        let tasks = get_all_tasks_at(&self.outbox);
        tasks.iter().for_each(|task| {
            if local_data.tasks.contains(&task) {
                let _ = local_data.add_task(&task);
            } else {
                let _ = local_data.update_task(task);
            }
        });

        Ok(tasks)
    }
}

impl Export for TWSync {
    fn export(&self) -> Result<Vec<Task>> {
        let dir_tree = get_dir_entries_recursively(&self.inbox)?;
        let mut tasks: Vec<Task> = Vec::new();
        dir_tree.iter().for_each(|entry| {
            let path = &entry.path();
            let name = path.file_name().expect("no name?");
            if path.is_file() && name.to_str().unwrap().contains("json") {
                if path.metadata().expect("bad metadata?").len() > 0 {
                    let task = read_from_json_file(entry.path().to_path_buf());
                    if entry.path().is_file() {
                        tasks.push(task.unwrap());
                    }
                }
            }
        });
        for task in &tasks {
            let json = task.to_json()?;
            let filename = self.inbox.display().to_string() + &task.uuid() + ".json";

            let mut file = File::create(filename)?;
            file.write_all(json.as_bytes())?;
        }

        Ok(tasks)
    }
}

pub fn read_from_json_file(path: PathBuf) -> Result<Task> {
    let json = fs::read_to_string(&path).expect("bad json file?");
    let tw26 = import_task::<TW26>(&json).expect(&format!("import failed? {}", path.display()));
    let task = Task::new(tw26);
    task
}
