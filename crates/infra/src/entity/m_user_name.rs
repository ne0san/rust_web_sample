use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "m_user_name")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub name: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    MNgWord,
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::MNgWord => Entity::belongs_to(super::m_ng_word::Entity)
                .from(Column::Name)
                .to(super::m_ng_word::Column::CreatedUserName)
                .into(),
        }
    }
}
impl ActiveModelBehavior for ActiveModel {}
