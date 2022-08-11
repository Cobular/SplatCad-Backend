#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use rocket::fairing::{self, AdHoc};
use rocket::{Build, Rocket};

use migration::MigratorTrait;

use sea_orm_rocket::Database;

mod errors;
mod guards;
mod pool;
mod routes;
use pool::Db;

use routes::debug::user_id;
use routes::project::{create, read_one, read_all_for_user};


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
        .mount("/debug", routes![user_id])
        .mount("/project", routes![create, read_one, read_all_for_user])
}
