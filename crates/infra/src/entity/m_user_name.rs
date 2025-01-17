use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "m_user_name")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    MNgWord,
    TPost,
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::MNgWord => Entity::belongs_to(super::m_ng_word::Entity)
                .from(Column::Id)
                .to(super::m_ng_word::Column::CreatedUserId)
                .into(),
            Self::TPost => Entity::belongs_to(super::t_post::Entity)
                .from(Column::Id)
                .to(super::t_post::Column::PostedUserId)
                .into(),
        }
    }
}

impl Related<super::t_post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TPost.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
