use crate::{page_result, query_tree_nodes};
use actix_web::web;

pub fn course_base_info_controller_init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/content")
            .service(page_result)
            .service(query_tree_nodes),
    );
}
