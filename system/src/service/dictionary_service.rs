use crate::model::{dictionary, dictionary::Entity as Dictionary};
use sea_orm::{
    ColIdx, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QuerySelect,
};

pub struct DictionaryService;

impl DictionaryService {
    pub async fn query_all(db: &DatabaseConnection) -> Result<Vec<dictionary::Model>, DbErr> {
        let list = Dictionary::find().all(db).await?;
        Ok(list)
    }
    pub async fn get_by_code(
        db: &DatabaseConnection,
        code: String,
    ) -> Result<dictionary::Model, DbErr> {
        let d = Dictionary::find()
            .filter(dictionary::Column::Code.eq(code))
            .one(db)
            .await?;
        match d {
            Some(d) => Ok(d),
            None => Err(DbErr::Json("not found".to_string())),
        }
    }
}
