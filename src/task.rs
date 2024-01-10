use task_hookrs::task::TW26;
use anyhow::Result;


#[derive(Debug)]
pub struct Task {
    tw26 :task_hookrs::task::Task<TW26>,
}

impl Task {
    pub fn uuid(&self) -> String {
        self.tw26.uuid().to_string()
    }

    pub fn project(&self) -> String {
        let project = self.tw26.project().unwrap().to_string();
        project
    }

    pub(crate) fn new(tw26: task_hookrs::task::Task) -> Task {
        Task {
            tw26
        }
    }

    pub(crate) fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string(&self.tw26)?;

        Ok(json)
    }

    pub(crate) fn to_toml(&self) -> Result<String> {
        todo!()
    }

    
}
