use crate::task::task_project::TaskProject;
use task_hookrs::date::Date;

use crate::task::{Deserialize, Serialize};

pub trait IsTaskField {}

#[derive(Clone,Deserialize, Serialize, Debug)]
pub struct TaskFieldString {
    field_name: TaskFieldName,
    value: Option<String>,
}

impl IsTaskField for TaskFieldString {}

impl TaskFieldString {
    pub fn new(field_name: TaskFieldName, value: String) -> TaskFieldString {
        TaskFieldString {
            field_name,
            value: Some(value),
        }
    }

    pub fn value(&self) -> &str {
        if self.value.is_some() {
            return self.value.as_ref().unwrap();
        } else {
            return "";
        }
    }
}
#[derive(Clone,Deserialize, Serialize, Debug)]
pub struct TaskFieldDate {
    field_name: TaskFieldName,
    value: Option<Date>,
}

impl IsTaskField for TaskFieldDate {}
impl TaskFieldDate {
    pub fn new(field_name: TaskFieldName, value: Option<Date>) -> TaskFieldDate {
        TaskFieldDate { field_name, value }
    }

    pub fn value(&self) -> Option<Date> {
        self.value.clone()
    }
}
#[derive(Clone,Deserialize, Serialize, Debug)]
pub struct TaskFieldProject {
    field_name: TaskFieldName,
    value: TaskProject,
}

impl TaskFieldProject {
    pub fn new(field_name: TaskFieldName, value: TaskProject) -> TaskFieldProject {
        TaskFieldProject { field_name, value }
    }

    pub fn value(&self) -> &TaskProject {
        &self.value
    }
}
#[derive(Clone,Deserialize, Serialize, Debug)]
pub enum TaskFieldName {
    UUID,
    Status,
    Description,
    Project,
    Entry,
    Wait,
    Scheduled,
    Due,
    Until,
    End,
    Urgency
}
