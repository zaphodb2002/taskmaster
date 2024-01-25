use anyhow::Result;

use crate::task::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskProject {
    campaign: String,
    aspect: Option<String>,
    project: Option<String>,
    subproject: Option<String>,
}

impl TaskProject {
    pub fn new(campaign :&str, aspect :Option<&str>, project :Option<&str>, subproject :Option<&str>) -> TaskProject {
       TaskProject {
           campaign: campaign.to_string(),
           aspect: aspect.map(|s| s.to_string()),
           project: project.map(|s| s.to_string()),
           subproject: subproject.map(|s| s.to_string())
       }
    }

    pub fn to_string(&self) -> String {
        let mut result :Vec<String> = vec!(self.campaign.clone());

        if self.aspect.is_some() {
            result.push(self.aspect.clone().unwrap());
        }

        if self.project.is_some() {
            result.push(self.project.clone().unwrap());
        }

        if self.subproject.is_some() {
            result.push(self.subproject.clone().unwrap());
        }


        result.join(".")
    }

    pub fn from_string(value: String) -> Result<TaskProject> {
        let mut parts = value.split(".");
        let campaign = parts.nth(0).expect("No project on task");
        let aspect = parts.nth(1);
        let project = parts.nth(2);
        let subproject = parts.nth(3);

        Ok(TaskProject::new(campaign, aspect, project, subproject))
        
    }
}
