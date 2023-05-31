use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
// 分页查询结果模型
#[derive(Deserialize, Serialize, Clone, Debug, ToSchema, IntoParams)]
pub struct PageResult<T> {
    items: Vec<T>,
    counts: i32,
    page_no: i32,
    page_size: i32,
}
impl<T> PageResult<T> {
    pub fn new(items: Vec<T>, counts: i32, page_no: i32, page_size: i32) -> Self {
        PageResult {
            items,
            counts,
            page_no,
            page_size,
        }
    }
}
