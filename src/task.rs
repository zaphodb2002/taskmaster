use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    TWTask :task_hookrs::task::Task,
}

impl Task {
    pub fn uuid(&self) -> String {
        self.TWTask.uuid().to_string()
    }

    pub fn project(&self) -> String {
        let project = self.TWTask.project().unwrap().to_string();
        project
    }

    pub(crate) fn new(tw26: task_hookrs::task::Task) -> Task {
        Task {
            TWTask: tw26
        }
    }

    
}
