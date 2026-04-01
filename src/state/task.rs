use std::sync::RwLock;
use std::collections::HashMap;
use serde::Serialize;

use crate::error::TaskError;


#[derive(Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: String, title: String, description: String,completed:bool) -> Self {
        Task { id, title, description,completed  }
    }
}


pub struct TaskStore{
    value: RwLock<HashMap<String, Task>>,
}

impl TaskStore{
    pub fn new()->Self{
        TaskStore{
            value: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_task(&self, task:Task){
        let mut tasks =self.value.write();
        match tasks {
            Ok(ref mut t) => t.insert(task.id.clone(), task),
            Err(_) => return, // Handle the error as needed
        };
    }
    pub fn get_task(&self, id:&str)->Option<Task>{
        let tasks = self.value.read();
        match tasks {
            Ok(ref t) => t.get(id).cloned(),
            Err(_) => None, // Handle the error as needed
        }

    }

    pub fn get_all_tasks(&self)->Vec<Task>{
        let tasks = self.value.read();
        match tasks {
            Ok(ref t) => t.values().cloned().collect(),
            Err(_) => Vec::new(), // Handle the error as needed
        }
    }
    pub fn complete_task(&self, id: &str) -> Result<Task, TaskError> {
        let mut tasks = self.value.write();
       match tasks {
            Ok(ref mut t) => {
                if let Some(task) = t.get_mut(id) {
                    task.completed = true;
                    Ok(task.clone())
                } else {
                    Err(TaskError::NotFound(id.to_string()))
                }
            },
            Err(_) => Err(TaskError::StoreError("Failed to access task store".into())), // Handle the error as needed
        }
    }
    
    pub fn update_task(&self, id: &str, title: Option<String>, description: Option<String>) -> Result<Task, TaskError> {
        let mut tasks = self.value.write();
        match tasks {
            Ok(ref mut t) => {
                if let Some(task) = t.get_mut(id) {
                    if let Some(new_title) = title {
                        task.title = new_title;
                    }
                    if let Some(new_description) = description {
                        task.description = new_description;
                    }
                    Ok(task.clone())
                } else {
                    Err(TaskError::NotFound(id.to_string()))
                }
            },
            Err(_) => Err(TaskError::StoreError("Failed to access task store".into())), // Handle the error as needed
        }
    }
    pub fn delete_task(&self, id: &str) -> Result<(), TaskError> {
        let mut tasks = self.value.write();
        match tasks {
            Ok(ref mut t) => {
                if t.remove(id).is_some() {
                    Ok(())
                } else {
                    Err(TaskError::NotFound(id.to_string()))
                }
            },
            Err(_) => Err(TaskError::StoreError("Failed to access task store".into())), // Handle the error as needed
        }
    }
}
