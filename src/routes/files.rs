
use entity::{projects, files};
use migration::JoinType;
use rocket::{serde::json::Json, response::status, http::Status};
use sea_orm::{EntityTrait, ColumnTrait, RelationTrait, QuerySelect, QueryFilter};
use sea_orm_rocket::Connection;

use crate::{pool::Db, guards::jwt::User};

#[get("/<project_id>/files")]
pub async fn get_files_for_project(
    conn: Connection<Db, '_>,
    user: User,
    project_id: i32,
) -> Result<Json<Vec<files::Model>>, status::Custom<String>> {
    let conn = conn.into_inner();
    match files::Entity::find()
        .join(JoinType::LeftJoin, files::Relation::Projects.def())
        .filter(projects::Column::CreatedBy.eq(user.id))
        .filter(files::Column::ProjectId.eq(project_id))
        .all(conn)
        .await
    {
        Ok(model) => Ok(rocket::serde::json::Json(model)),
        _ => Err(status::Custom(
            Status::InternalServerError,
            "Could not find project".to_string(),
        )),
    }
}
