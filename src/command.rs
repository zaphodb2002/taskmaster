use crate::{task::Task, taskpool::TaskPool};

pub enum Command {
    Add,
    Remove,
    Modify,
    Report,
    Import,
    Export,
    Help,
    Version,
    NotRecognized,
}

impl Command {
    pub(crate) fn from(str: String) -> Command {
        match str.as_str() {
            "add" => Command::Add,
            "remove" => Command::Remove,
            "modify" => Command::Modify,
            "report" => Command::Report,
            "import" => Command::Import,
            "export" => Command::Export,
            "help" => Command::Help,
            "version" => Command::Version,
            _ => Command::NotRecognized,
        }
    }
}

pub struct CommandResult {
    pub tasks: TaskPool,
    pub text: Vec<String>,
}
