use crate::state::{ Task, TaskStore };
use actix_web::{ web, Responder, HttpResponse };

#[derive(serde::Deserialize)]
pub struct UpdateRequest {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

pub async fn update_task(
    req: web::Json<UpdateRequest>,
    store: web::Data<TaskStore>
) -> impl Responder {
    // 1. Call your update_task method
    // 2. Handle the Result (Return 200 OK with the task, or 404 Not Found)
    let input = req.into_inner();
    match store.update_task(&input.id, input.title, input.description) {
        Ok(updated_task) => HttpResponse::Ok().json(updated_task),
        Err(_) => HttpResponse::NotFound().body("Task not found"),
    }
}
