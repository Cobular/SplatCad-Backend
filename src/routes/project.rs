use crate::{guards::jwt::User, pool::Db};
use entity::{files, projects};
use migration::JoinType;
use rocket::{http::Status, response::status, serde::json::Json};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, RelationTrait,
};
use sea_orm_rocket::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MiniModel {
    pub name: String,
    pub description: Option<String>,
    pub enforce_checkouts: Option<bool>,
}

impl MiniModel {
    pub fn into_model(self, user: User) -> projects::ActiveModel {
        let desc = match self.description {
            Some(desc) => ActiveValue::set(Some(desc)),
            None => ActiveValue::NotSet,
        };

        projects::ActiveModel {
            name: ActiveValue::Set(self.name),
            description: desc,
            enforce_checkouts: ActiveValue::Set(self.enforce_checkouts.unwrap_or(false)),
            created_by: ActiveValue::Set(user.id),
            ..Default::default()
        }
    }
}

#[post("/", format = "application/json", data = "<project>")]
pub async fn create(
    conn: Connection<Db, '_>,
    user: User,
    project: Json<MiniModel>,
) -> Result<Json<projects::Model>, status::Custom<String>> {
    let conn = conn.into_inner();

    // Check if one exists before we make a second one
    match projects::Entity::find()
        .filter(projects::Column::Name.eq(project.name.clone()))
        .filter(projects::Column::CreatedBy.eq(user.id))
        .count(conn)
        .await
    {
        Ok(count) => {
            if count > 0 {
                return Err(status::Custom(
                    Status::BadRequest,
                    "A project with that name already exists".to_string(),
                ));
            }
        }
        Err(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                "Could not create project".to_string(),
            ))
        }
    };

    match project.into_inner().into_model(user).insert(conn).await {
        Ok(model) => Ok(rocket::serde::json::Json(model)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Could not create project".to_string(),
        )),
    }
}

#[get("/<id>")]
pub async fn read_one(
    conn: Connection<Db, '_>,
    _user: User,
    id: i32,
) -> Result<Json<projects::Model>, status::Custom<String>> {
    let conn = conn.into_inner();
    match projects::Entity::find_by_id(id).one(conn).await {
        Ok(Some(model)) => Ok(rocket::serde::json::Json(model)),
        _ => Err(status::Custom(
            Status::InternalServerError,
            "Could not find project".to_string(),
        )),
    }
}

#[get("/")]
pub async fn read_all_for_user(
    conn: Connection<Db, '_>,
    user: User,
) -> Result<Json<Vec<projects::Model>>, status::Custom<String>> {
    let conn = conn.into_inner();
    match projects::Entity::find()
        .filter(projects::Column::CreatedBy.eq(user.id))
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

#[get("/<project_id>/commits")]
pub async fn get_commits_for_project(
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
