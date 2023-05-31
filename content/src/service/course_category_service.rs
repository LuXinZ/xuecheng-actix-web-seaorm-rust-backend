use crate::model::{course_category, course_category::Entity as CourseCategory};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, UpdateResult};

pub struct CourseCategoryService;
impl CourseCategoryService {
    pub async fn query_tree_nodes(
        db: &DatabaseConnection,
    ) -> Result<Vec<course_category::Model>, DbErr> {
        // 找出所有分类
        let all_course_categories = CourseCategory::find().all(db).await?;
        // 组装成父子树结构， 找出所有一级分类
        let mut level1_menus = all_course_categories
            .iter()
            .filter(|category| category.parentid.eq("1"))
            .map(|menu| {
                let mut menu = menu.clone();
                menu.children_tree_nodes = get_childrens(&menu, &all_course_categories);
                menu
            })
            .collect::<Vec<course_category::Model>>();
        level1_menus.sort_by(|a, b| Some(a.orderby).cmp(&Some(b.orderby)));
        Ok(level1_menus)
    }
}
fn get_childrens(
    menu: &course_category::Model,
    category_entities: &Vec<course_category::Model>,
) -> Vec<course_category::Model> {
    let mut menu1 = category_entities
        .iter()
        .filter(|category| category.parentid.eq(&menu.id))
        .map(|category| {
            let mut category = category.clone();
            category.children_tree_nodes = get_childrens(&category, category_entities);
            category
        })
        .collect::<Vec<course_category::Model>>();
    menu1.sort_by(|a, b| Some(a.orderby).cmp(&Some(b.orderby)));
    menu1
}
