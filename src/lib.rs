use actix_web::web;

use crate::state::TaskStore;

pub mod error;
pub mod state;
pub mod instructions;
pub mod constants;

use instructions::add_task::add_task;
use instructions::complete_task::complete_task;
use instructions::get_task::get_task;
// use instructions::get_one::get_one;
use instructions::update_task::update_task;
use instructions::delete_task::delete_task;

pub fn configure(cfg: &mut web::ServiceConfig, store: web::Data<TaskStore>) {
  cfg.service(
        web::scope("/api")
            .route("/add_task", web::post().to(add_task))       // CREATE
            .route("/tasks", web::get().to(get_task))           // READ ALL
            // .route("/get_task/{id}", web::get().to(get_one))    // READ ONE
            .route("/update_task", web::put().to(update_task))  // UPDATE
            .route("/delete_task/{id}", web::delete().to(delete_task)) // DELETE
    );
    
}
