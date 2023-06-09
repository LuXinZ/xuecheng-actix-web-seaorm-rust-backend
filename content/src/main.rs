use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use common::{dev_cors, AppState};
use content::course_base_info_controller_init;
use sea_orm::{Database, DatabaseConnection};
use serde_json::Value;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

const API_KEY_NAME: &str = "todo_apikey";
const API_KEY: &str = "utoipa-rocks";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: DatabaseConnection = Database::connect("mysql://root:root@localhost:3306/xc_content")
        .await
        .expect("Failed to connect to database");
    let state = AppState { db };
    #[derive(OpenApi)]
    #[openapi(
    paths( content::page_result,),
    components(
    schemas(common::PageParams)
    ),
    tags(
    (name = "content", description = "在线内容管理系统")
    ),
    modifiers()
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .wrap(dev_cors())
            .app_data(web::Data::new(state.clone()))
            .configure(course_base_info_controller_init)
    })
    .bind(("127.0.0.1", 63040))?
    .run()
    .await
}
