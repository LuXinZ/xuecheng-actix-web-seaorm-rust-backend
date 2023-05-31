use crate::CourseCategoryService;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use common::{handle_response, AppResult, AppState};

// 课程分类相关接口
#[get("/course-category/tree-nodes")]
pub async fn query_tree_nodes(data: web::Data<AppState>) -> AppResult {
    let res = CourseCategoryService::query_tree_nodes(&data.db).await;
    handle_response(res)
}
