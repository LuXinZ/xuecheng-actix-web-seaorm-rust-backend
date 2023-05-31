use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
#[derive(Deserialize, Serialize, Clone, Debug, ToSchema, IntoParams)]
pub struct QueryCourseParamsDto {
    // 审核状态
    pub audit_status: String,
    // 课程名称
    pub course_name: String,
    // 发布状态
    pub publish_status: String,
}
