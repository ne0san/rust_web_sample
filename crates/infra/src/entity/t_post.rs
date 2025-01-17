use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "t_post")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub posted_user_id: i32,
    pub content: String,
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
                .from(Column::PostedUserId)
                .to(super::m_user_name::Column::Id)
                .into(),
        }
    }
}
impl Related<super::m_user_name::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MUserName.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
