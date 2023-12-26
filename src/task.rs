use serde::{Deserialize, Serialize};
use std::fs;
use substring::Substring;
use time::PrimitiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub(crate) uuid: String,
    pub(crate) description: String,
    pub(crate) tags: Vec<String>,
    pub(crate) mask: String,
    pub(crate) modified: Option<PrimitiveDateTime>,
    pub(crate) project: Vec<String>,
    pub(crate) rtype: String,
    pub(crate) recur: String,
    pub(crate) status: String,
    pub(crate) entry: Option<PrimitiveDateTime>,
    pub(crate) wait: Option<PrimitiveDateTime>,
    pub(crate) scheduled: Option<PrimitiveDateTime>,
    pub(crate) start: Option<PrimitiveDateTime>,
    pub(crate) due: Option<PrimitiveDateTime>,
    pub(crate) until: Option<PrimitiveDateTime>,
    pub(crate) end: Option<PrimitiveDateTime>,
}

impl Task {
    pub(crate) fn write(&self) -> () {
        let folderpath = &self.create_folder();
        let fullpath = folderpath.to_owned() + &self.uuid + ".json";
        let json = serde_json::to_string(&self).unwrap();
        fs::write(fullpath, json);
    }
    fn create_folder(&self) -> String {
        let mut builder = fs::DirBuilder::new();
        builder.recursive(true);

        let mut path = "./pages/".to_string();
        for level in &self.project {
            path += level;
            path += "/";
        }
        builder.create(path.clone()).unwrap();

        path
    }

    pub fn format_for_report(&self) -> String {
        let uuid = &format_uuid(&self.uuid);
        let description = &format_description(&self.description);
        let status = &self.status;

        let entry = &format_date(&self.entry);
        let wait = &format_date(&self.wait);
        //let scheduled = format_date(&self.scheduled);
        //let start = &format_date*&self.start;
        let due = &format_date(&self.due);
        //let until = format_date(&self.until);
        let end = &format_date(&self.end);

        let mut result = String::new();
        result += uuid;
        result += REPORT_SEPARATOR;
        result += description;
        result += REPORT_SEPARATOR;
        result += status;
        result += REPORT_SEPARATOR;
        result += entry;
        result += REPORT_SEPARATOR;
        result += wait;
        result += REPORT_SEPARATOR;
        result += due;
        result += REPORT_SEPARATOR;
        result += end;

        result
    }
}

const REPORT_SEPARATOR: &str = " | ";

fn format_uuid(uuid: &str) -> String {
    uuid.to_string().substring(0, 8).to_string()
}

fn format_description(desc: &str) -> String {
    desc.replace("\"", "").replace("\\", "")
}

fn format_date(date: &Option<PrimitiveDateTime>) -> String {
    if date.is_none() {
        return "No Data".to_string();
    }
    let date = date.unwrap();
    date.to_string()
}

pub const EXAMPLE_0: &str = r#"
    {
        "description":"AM Check-In",
        "due":"20230821T160000Z",
        "entry":"20230822T183835Z",
        "mask":"++++++++++++++++++W",
        "modified":"20230907T193905Z",
        "project":"LMS.Process",
        "recur":"1d",
        "rtype":"periodic",
        "status":"recurring",
        "uuid":"1e0b95c1-c9e7-49e7-9363-aaa62c17623a",
        "wait":"20231005T155005Z",
        "tags":["am","daily"]
    }
    "#;

pub const EXAMPLE_1: &str = r#"
    {
        "description":"AM Check-In",
        "due":"20230821T160000Z",
        "entry":"20230822T183835Z",
        "mask":"++++++++++++++++++W",
        "modified":"20230907T193905Z",
        "project":"LMS.Process",
        "recur":"1d",
        "rtype":"periodic",
        "status":"recurring",
        "uuid":"1e0b95c1-c9e7-49e7-9363-aaa62c17623a",
        "wait":"20231005T155005Z",
        "tags":["am","daily"]
    }
    "#;

// TODO: implement new()
// TODO: implement get_by_uuid()
// TODO: implement modify()
