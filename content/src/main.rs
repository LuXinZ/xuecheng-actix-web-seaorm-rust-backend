use actix_web::{App, HttpServer, web};
use sea_orm::{Database, DatabaseConnection};
use common::{AppState};
#[actix_web::main]
async fn main() ->std::io::Result<()>{
    println!("Hello, world!");
    let db: DatabaseConnection = Database::connect("mysql://root:root@localhost:3306/xuecheng_content").await.expect("Failed to connect to database");
    let state = AppState { db };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(init)
    })

        .bind(("127.0.0.1", 8090))?
        .run()
        .await
}
fn init(cfg: &mut web::ServiceConfig) {


}
