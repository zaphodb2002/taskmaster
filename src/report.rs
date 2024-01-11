use crate::task::Task;
use anyhow::Result;

pub struct Report {
    tasks :Vec<Task>
}

impl Report {
    pub fn full() -> Result<Report> {
        todo!()
    }

    pub(crate) fn to_string(&self) -> String {
        todo!()
    }

    pub(crate) fn import(tasks: Vec<Task>) -> String {
        let mut strings :Vec<String> = Vec::new();
        tasks.iter().for_each(|task|{
            strings.push(format!("{} imported", task.uuid().unwrap()));
        });
        
        strings.join("\n")
    }

    pub(crate) fn export(tasks: Vec<Task>) -> String {
        let mut strings :Vec<String> = Vec::new();
        tasks.iter().for_each(|task|{
            strings.push(format!("{} exported", task.uuid().unwrap()));
        });
        
        strings.join("\n")
    }
}
