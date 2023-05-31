use actix_web::{web, App, HttpServer};
use common::AppState;
use sea_orm::{Database, DatabaseConnection};
use system::dictionary_controller_init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: DatabaseConnection = Database::connect("mysql://root:root@localhost:3306/xc_system")
        .await
        .expect("Failed to connect to database");
    let state = AppState { db };
    HttpServer::new(move || {
        App::new()
            .configure(dictionary_controller_init)
            .app_data(web::Data::new(state.clone()))
    })
    .bind(("127.0.0.1", 63110))?
    .run()
    .await
}
