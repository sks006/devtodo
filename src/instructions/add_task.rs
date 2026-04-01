use crate::state::{Task, TaskStore};
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Request {
    pub title: String,
    pub description: String,
}

pub async fn add_task(request: web::Json<Request>, task_store: web::Data<TaskStore>)-> HttpResponse {
    let input = request.into_inner();
    let id = uuid::Uuid::new_v4().to_string();
    let task = Task::new(id, input.title, input.description, false);
    task_store.add_task(task);
    HttpResponse::Ok().body("Task added successfully")
}
