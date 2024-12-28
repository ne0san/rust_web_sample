use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "m_ng_word")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub word: String,
    pub created_user_id: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    MUserName,
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::MUserName => Entity::belongs_to(super::m_user_name::Entity)
                .from(Column::CreatedUserId)
                .to(super::m_user_name::Column::Id)
                .into(),
        }
    }
}
impl ActiveModelBehavior for ActiveModel {}
