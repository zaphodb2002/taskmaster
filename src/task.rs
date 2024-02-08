use anyhow::Result;
use serde::{Deserialize, Serialize};
use task_hookrs::date::Date;

mod task_field;
use task_field::*;

mod task_project;
use task_project::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    uuid: TaskFieldString,
    status: TaskFieldString,
    description: TaskFieldString,
    project: TaskFieldProject,
    entry: TaskFieldDate,
    wait: TaskFieldDate,
    scheduled: TaskFieldDate,
    due: TaskFieldDate,
    until: TaskFieldDate,
    end: TaskFieldDate,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.uuid() == other.uuid()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Task {
    pub fn uuid(&self) -> String {
        self.uuid.value().to_string()
    }
    pub fn status(&self) -> String {
        self.status.value().to_string()
    }

    pub fn description(&self) -> String {
        self.description.value().to_string()
    }

    pub fn project(&self) -> &TaskProject {
        self.project.value()
    }

    pub fn wait(&self) -> Option<String> {
        let result = Self::format_date(self.until.value().as_ref());
        Some(result)
    }

    pub fn scheduled(&self) -> Option<String> {
        let result = Self::format_date(self.scheduled.value().as_ref());
        Some(result)
    }

    pub fn due(&self) -> Option<String> {
        let result = Self::format_date(self.due.value().as_ref());
        Some(result)
    }

    pub fn until(&self) -> Option<String> {
        let result = Self::format_date(self.until.value().as_ref());
        Some(result)
    }

    pub(crate) fn new(tw26: task_hookrs::task::Task) -> Result<Task> {
        Ok(Task {
            uuid: TaskFieldString::new(TaskFieldName::UUID, tw26.uuid().to_string()),
            status: TaskFieldString::new(TaskFieldName::Status, tw26.status().to_string()),
            description: TaskFieldString::new(
                TaskFieldName::Description,
                tw26.description().to_string(),
            ),
            project: TaskFieldProject::new(
                TaskFieldName::Project,
                TaskProject::from_string(tw26.project().expect("Bad project").to_string())?,
            ),
            entry: TaskFieldDate::new(TaskFieldName::Entry, Some(tw26.entry().to_owned())),
            wait: TaskFieldDate::new(TaskFieldName::Wait, tw26.wait().cloned()),
            scheduled: TaskFieldDate::new(TaskFieldName::Scheduled, tw26.scheduled().cloned()),
            due: TaskFieldDate::new(TaskFieldName::Due, tw26.due().cloned()),
            until: TaskFieldDate::new(TaskFieldName::Until, tw26.until().cloned()),
            end: TaskFieldDate::new(TaskFieldName::End, tw26.end().cloned())
        })
    }

    pub(crate) fn to_md(&self) -> Result<String> {
        let title = format!("# {}\n", self.uuid());
        let description = format!("{}\n", self.description());
        let project = format!("# {}\n", self.project().to_string());
        let wait = format!("wait: {}\n", self.wait().unwrap());
        let scheduled = format!("scheduled: {}\n", self.scheduled().unwrap());
        let due = format!("due: {}\n", self.due().unwrap());
        let until = format!("until: {}\n", self.until().unwrap());

        let md = format!(
            "{}{}{}{}{}{}{}",
            title, description, project, wait, scheduled, due, until,
        );
        Ok(md)
    }

    pub(crate) fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string(&self)?;

        Ok(json)
    }

    pub(crate) fn to_toml(&self) -> Result<String> {
        let toml = toml::to_string(&self)?;
        Ok(toml)
    }

    fn format_date(date: Option<&Date>) -> String {
        if date.is_some() {
            return date.unwrap().to_string();
        }
        String::new()
    }
}
