use crate::state::{Task, TaskStore};
use actix_web::{web, Responder, HttpResponse};

pub async fn delete_task(
    store: web::Data<TaskStore>,
    id: web::Path<String>
) -> impl Responder {
    let id = id.into_inner();
    match store.delete_task(&id) {
        Ok(()) => HttpResponse::Ok().body("Task deleted"),
        Err(_) => HttpResponse::NotFound().body("Task not found"),
    }
}
