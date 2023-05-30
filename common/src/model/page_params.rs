use serde::{Deserialize, Serialize};
use utoipa::{ToSchema,IntoParams};

#[derive(Deserialize, Serialize, Clone, Debug,ToSchema,IntoParams)]
pub struct PageParams {
    // 页码
    pub page_no: i32,
    // 每页记录数
    pub page_size: i32,
}
impl Default for PageParams {
    fn default() -> Self {
        PageParams {
            page_no: 1,
            page_size: 10,
        }
    }
}
impl PageParams {
    pub fn new(page_no: i32, page_size: i32) -> Self {
        PageParams { page_no, page_size }
    }
}
