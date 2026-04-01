use actix_web::{web, Responder, HttpResponse};

use crate::state::TaskStore;

pub async fn get_task(task_store:web::Data<TaskStore>)-> impl Responder {
    // 1. Get the data from the engine
    let tasks = task_store.get_all_tasks();
println!("DEBUG: Current tasks in store: {:?}", tasks);
    // 2. Wrap it in the "JSON" delivery crate and return it
    // How do you write the return line here?
    HttpResponse::Ok().json(tasks)

}