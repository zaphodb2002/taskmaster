use crate::{taskpool::TaskPool, task::Task};

pub struct Report {
    headers: Vec<String>,
    pub taskpool: TaskPool
}

impl Report {
    pub fn gtd() -> Report {
        let taskpool = TaskPool::get_all_tasks();
        dbg!(taskpool.tasks.len());
        let mut filtered_tasks: Vec<Task> = Vec::new();
                      
        filtered_tasks.append(&mut taskpool.get_tasks_by_project("LMS".to_string()).tasks);
        filtered_tasks = filtered_tasks.into_iter()
            .filter(|t| t.end == None)
            .collect();
        Report {
            headers: ["uuid".to_string()].to_vec(),
            taskpool: TaskPool{
                tasks: filtered_tasks 
            }
        }
    }

    pub fn format(&self) -> Vec<String> {
        let mut result :Vec<String> = Vec::new();
        for task in &self.taskpool.tasks {
            result.push(task.description.clone());
        }
       result
    }
}

fn get_tasks_for_gtd_report(tasks: &Vec<Task>) -> Vec<Task> {
    let mut result: Vec<Task> = Vec::new();
    panic!("Not Yet Implemented");
    result
}


