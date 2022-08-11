//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "commits")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub project_id: i32,
    pub description: Option<String>,
    pub created_by: i32,
    pub created_at: DateTimeWithTimeZone,
    pub commit_number: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::CreatedBy",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users,
    #[sea_orm(
        belongs_to = "super::projects::Entity",
        from = "Column::ProjectId",
        to = "super::projects::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Projects,
    #[sea_orm(has_many = "super::versions::Entity")]
    Versions,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl Related<super::versions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Versions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
