use time::PrimitiveDateTime;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Task {
    pub(crate) description :String,
    pub(crate) due :Option<PrimitiveDateTime>,
    pub(crate) entry :Option<PrimitiveDateTime>,
    pub(crate) mask :String,
    pub(crate) modified :Option<PrimitiveDateTime>,
    pub(crate) project :Vec<String>,
    pub(crate) recur :String,
    pub(crate) rtype :String,
    pub(crate) status: String,
    pub(crate) uuid :String,
    pub(crate) wait :Option<PrimitiveDateTime>,
    pub(crate) tags :Vec<String>,
    pub(crate) end :Option<PrimitiveDateTime>,
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
}


pub const EXAMPLE_0 :&str = r#"
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

pub const EXAMPLE_1 :&str = r#"
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
// TODO: implement 
