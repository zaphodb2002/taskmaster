use time::PrimitiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Task {
    pub(crate) description :String,
    pub(crate) due :Option<PrimitiveDateTime>,
    pub(crate) entry :Option<PrimitiveDateTime>,
    pub(crate) mask :String,
    pub(crate) modified :Option<PrimitiveDateTime>,
    pub(crate) project :String,
    pub(crate) recur :String,
    pub(crate) rtype :String,
    pub(crate) status: String,
    pub(crate) uuid :String,
    pub(crate) wait :Option<PrimitiveDateTime>,
    pub(crate) tags :Vec<String>,

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
