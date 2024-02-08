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
        let all :Vec<Task> = LocalData::new(MD_FILE_ROOT.into())?.tasks
            .into_iter()
            .filter(|task| task.status() == "Pending")
            .collect();
        
        let mut campaigns :Vec<&str> = all.iter()
            .map(|task| task.project().campaign())
            .collect();
        campaigns.sort();
        campaigns.dedup();
        
        let mut tasks :Vec<Task> = Vec::new();

        campaigns.into_iter().for_each(|campaign| {
            let new_task = all.iter()
                .filter(|task| task.project().campaign() == campaign)
                .nth(0).unwrap();
            tasks.push(new_task.clone());
        });


        let result = Report {
            title: "Getting Things Done".into(),
            tasks,
            columns: vec![
                "description".into(), 
                "status".into()
            ],
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
