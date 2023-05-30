//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "teachplan_media")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub media_id: Option<String>,
    pub teachplan_id: i64,
    pub course_id: i64,
    #[sea_orm(column_name = "media_fileName")]
    pub media_file_name: String,
    pub create_date: Option<DateTime>,
    pub create_people: Option<String>,
    pub change_people: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}