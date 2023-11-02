use crate::task::Task;

#[derive(Debug)]
pub(crate) struct TaskPool(pub Vec<Task>);

