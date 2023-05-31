use crate::model::{course_base, course_base::Entity as CourseBase};
use crate::QueryCourseParamsDto;
use common::PageParams;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QuerySelect};

pub struct CourseBaseInfoService;
impl CourseBaseInfoService {
    // 课程分页查询
    pub async fn query_course_base_list(
        db: &DatabaseConnection,
        page_params: PageParams,
        query_course_params_dto: QueryCourseParamsDto,
    ) -> Result<Vec<course_base::Model>, DbErr> {
        // 根据名称模糊查询, 根据课程审核状态查询， 按照课程发布状态查询
        let res = CourseBase::find()
            .filter(
                course_base::Column::Name.contains(query_course_params_dto.course_name.as_str()),
            )
            .filter(
                course_base::Column::AuditStatus.eq(query_course_params_dto.audit_status.as_str()),
            )
            .filter(course_base::Column::Status.eq(query_course_params_dto.publish_status.as_str()))
            .offset(Some(
                (page_params.page_no - 1) as u64 * page_params.page_size as u64,
            ))
            .limit(Some(page_params.page_size as u64))
            .all(db)
            .await?;
        Ok(res)
    }
}
