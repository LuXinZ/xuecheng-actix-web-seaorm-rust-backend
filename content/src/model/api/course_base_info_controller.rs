use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use common::{AppResult, AppState, MessageResponse, PageParams};

pub fn course_base_info_controller_init(cfg: &mut web::ServiceConfig) {
    cfg.service(page_result);
}
#[utoipa::path(
post,
path = "/course/list/{page_no}/{page_size}",
responses((status = 200, description = "Pet found succesfully",),),
params(PageParams)
)]
#[post("/course/list/{page_no}/{page_size}")]
pub async fn page_result(info: web::Path<PageParams>, data: web::Data<AppState>) -> AppResult {
    Ok(MessageResponse::new(None))
}
