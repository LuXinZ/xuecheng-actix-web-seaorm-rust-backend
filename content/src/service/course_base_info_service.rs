use crate::model::{course_base, course_base::Entity as CourseBase};
use crate::QueryCourseParamsDto;
use common::{PageParams, PageResult};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};

pub struct CourseBaseInfoService;
impl CourseBaseInfoService {
    // 课程分页查询
    pub async fn query_course_base_list(
        db: &DatabaseConnection,
        page_params: PageParams,
        query_course_params_dto: QueryCourseParamsDto,
    ) -> Result<PageResult<course_base::Model>, DbErr> {
        // 根据名称模糊查询, 根据课程审核状态查询， 按照课程发布状态查询
        let mut query = CourseBase::find();
        if !query_course_params_dto.audit_status.is_empty() {
            query = query.filter(
                course_base::Column::AuditStatus.eq(query_course_params_dto.audit_status.as_str()),
            )
        }
        if !query_course_params_dto.publish_status.is_empty() {
            query = query.filter(
                course_base::Column::Status.eq(query_course_params_dto.publish_status.as_str()),
            )
        }
        if !query_course_params_dto.course_name.is_empty() {
            query = query.filter(
                course_base::Column::Name.contains(query_course_params_dto.course_name.as_str()),
            )
        }
        let p = query.paginate(db, page_params.page_size as u64);
        let count = p.num_items().await?;
        let r = p.fetch_page((page_params.page_no - 1) as u64).await?;
        Ok(PageResult::new(
            r,
            count as i32,
            page_params.page_no,
            page_params.page_size,
        ))
    }
}
