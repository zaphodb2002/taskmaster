use task_hookrs::{task::TW26, date::Date};
use anyhow::Result;


#[derive(Debug)]
pub struct Task {
    tw26 :task_hookrs::task::Task<TW26>,
}

impl Task {
    pub fn uuid(&self) -> Result<String> {
        Ok(self.tw26.uuid().to_string())
    }

    
    pub fn status(&self) -> Result<String> {
        Ok(self.tw26.status().to_string())
    }

    pub fn description(&self) -> Result<String> {
        Ok(self.tw26.description().to_string())
    }

    pub fn project(&self) -> Result<String> {
        Ok(self.tw26.project().unwrap().to_string())
    }

    pub fn wait(&self) -> Result<String> {
        let result = Self::format_date(self.tw26.until());
        Ok(result)
    }

    pub fn scheduled(&self) -> Result<String> {
        let result = Self::format_date(self.tw26.scheduled());
        Ok(result)
    }

    pub fn due(&self) -> Result<String> {
        let result = Self::format_date(self.tw26.due());
        Ok(result)
    }

    pub fn until(&self) -> Result<String> {
        let result = Self::format_date(self.tw26.until());
        Ok(result)
    }

    pub(crate) fn new(tw26: task_hookrs::task::Task) -> Task {
        Task {
            tw26
        }
    }

    pub(crate) fn to_md(&self) -> Result<String> {
        let title = format!("# {}\n", self.uuid()?);
        let description = format!("{}\n", self.description()?);
        let project = format!("# {}\n", self.project()?);
        let wait = format!("wait: {}\n", self.wait()?);
        let scheduled = format!("scheduled: {}\n", self.scheduled()?);
        let due = format!("due: {}\n", self.due()?);
        let until = format!("until: {}\n", self.until()?);

        let md = format!(
            "{}{}{}{}{}{}{}",
            title,
            description,
            project,
            wait,
            scheduled,
            due,
            until,
            );
        Ok(md)
    }

    pub(crate) fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string(&self.tw26)?;

        Ok(json)
    }

    pub(crate) fn to_toml(&self) -> Result<String> {
        todo!()
    }

    fn format_date(date :Option<&Date>) -> String{
        if date.is_some(){
            return date.unwrap().to_string();
        }
        String::new()
    }
}
