//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "course_market")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub charge: String,
    #[sea_orm(column_type = "Float", nullable)]
    pub price: Option<f32>,
    #[sea_orm(column_type = "Float", nullable)]
    pub original_price: Option<f32>,
    pub qq: Option<String>,
    pub wechat: Option<String>,
    pub phone: Option<String>,
    pub valid_days: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
