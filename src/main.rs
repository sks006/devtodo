use devtodo::state::TaskStore;
// use crate::instructions::{ add_task, complete_task };
use actix_web::{ web, App, HttpServer };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Initialize the Master Store
    let task_store = web::Data::new(TaskStore::new());

    println!("🏎️ Engine Started: Listening on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            // 🚨 THIS IS THE FIX: This makes TaskStore available to all handlers
            .app_data(task_store.clone()) 
            // This just sets up the routes
            .configure(|cfg| devtodo::configure(cfg, task_store.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


