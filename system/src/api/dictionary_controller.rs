use crate::DictionaryService;
use actix_web::web::to;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use common::{handle_response, AppResult, AppState};

pub fn dictionary_controller_init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/system")
            .service(query_all)
            .service(get_by_code),
    );
}
#[get("/dictionary/all")]
async fn query_all(data: web::Data<AppState>) -> AppResult {
    let res = DictionaryService::query_all(&data.db).await;
    handle_response(res)
}
#[get("/dictionary/code/{code}")]
async fn get_by_code(data: web::Data<AppState>, code: web::Path<String>) -> AppResult {
    let res = DictionaryService::get_by_code(&data.db, code.into_inner()).await;
    handle_response(res)
}
