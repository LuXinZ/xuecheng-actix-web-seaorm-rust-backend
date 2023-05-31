use crate::{CourseBaseInfoService, QueryCourseParamsDto};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use common::{handle_response, AppResult, AppState, MessageResponse, PageParams};

#[utoipa::path(
post,
path = "/course/list/{page_no}/{page_size}",
responses((status = 200, description = "et found succesfully",),),
params(PageParams)
)]
#[post("/course/list")]
pub async fn page_result(
    info: web::Query<PageParams>,
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
