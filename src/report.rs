use crate::{
    sync::local_data::{LocalData, MD_FILE_ROOT},
    task::Task,
};
use anyhow::Result;

mod gtd;

pub struct Report {
    title: String,
    tasks: Vec<Task>,
    columns: Vec<String>,
}

impl Report {
    pub fn gtd() -> Result<Report> {
        let tasks = LocalData::new(MD_FILE_ROOT.into())?.tasks;

        let result = Report {
            title: "Getting Things Done".into(),
            tasks,
            columns: vec!["uuid".into(), "status".into()],
        };

        Ok(result)
    }

    pub(crate) fn import(tasks: Vec<Task>) -> String {
        let mut strings: Vec<String> = Vec::new();
        tasks.iter().for_each(|task| {
            strings.push(format!("{} imported", task.uuid()));
        });

        strings.join("\n")
    }

    pub(crate) fn export(tasks: Vec<Task>) -> String {
        let mut strings: Vec<String> = Vec::new();
        tasks.iter().for_each(|task| {
            strings.push(format!("{} exported", task.uuid()));
        });

        strings.join("\n")
    }

    pub(crate) fn display(&self) -> String {
        let header_line = self.columns.join(" | ");

        let mut result = format!("{}\n| {} |\n", self.title, header_line);

        for task in &self.tasks {
            let mut line = String::new();
            for field in &self.columns {
                let s = match field.as_str() {
                    "uuid" => task.uuid(),
                    "description" => task.description(),
                    "status" => task.status(),
                    _ => "ERROR".to_string(),
                };

                line += format!("| {} ", s).as_str();
            }
            line += "|\n";
            result += line.as_str();
        }
        result
    }
}
