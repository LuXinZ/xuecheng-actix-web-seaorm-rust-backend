use crate::{CourseBaseInfoService, QueryCourseParamsDto};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use common::{handle_response, AppResult, AppState, MessageResponse, PageParams};

pub fn course_base_info_controller_init(cfg: &mut web::ServiceConfig) {
    cfg.service(page_result);
}
#[utoipa::path(
post,
path = "/course/list/{page_no}/{page_size}",
responses((status = 200, description = "et found succesfully",),),
params(PageParams)
)]
#[post("/course/list/{page_no}/{page_size}")]
async fn page_result(
    info: web::Path<PageParams>,
    course: web::Json<QueryCourseParamsDto>,
    data: web::Data<AppState>,
) -> AppResult {
    let list = CourseBaseInfoService::query_course_base_list(
        &data.db,
        info.into_inner(),
        course.into_inner(),
    )
    .await;

    handle_response(list)
}
