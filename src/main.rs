#[macro_use] extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::{Build, Rocket};
use serde_json::json;

use migration::MigratorTrait;
use sea_orm::{entity::*, query::*};
use sea_orm_rocket::{Connection, Database};

mod pool;
mod jwt;
use pool::Db;

pub use entity::post;
pub use entity::post::Entity as Post;

use crate::jwt::User;

#[get("/")]
fn index(user: User) -> String {
    user.sub
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Db::init())
    .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
    .mount("/", routes![index])
}