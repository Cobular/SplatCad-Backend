use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, Validation};
use migration::{DbErr, OnConflict};
use once_cell::sync::Lazy;
use rocket::{
    http::Status,
    outcome::IntoOutcome,
    request::{self, FromRequest, Outcome},
    Config, Request,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use sea_orm_rocket::Connection;
use serde::{Deserialize, Serialize};

use entity::users::{Entity as User, Model as UserModel};

use crate::pool::Db;

#[derive(Deserialize)]
struct JwtConfig {
    pub secret: String,
}

static JWT_CONFIG: Lazy<JwtConfig> = Lazy::new(|| {
    Config::figment()
        .extract::<JwtConfig>()
        .expect("Failed to load JWT config, check Rocket.toml")
});

static VALIDATION: Lazy<Validation> = Lazy::new(|| {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["https://splatcad-api.cobular.com"]);
    validation
});

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaim {
    pub sub: String,
}

/// Validates JWT and ensures that User is in the DB
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserClaim {
    type Error = ErrorKind;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = match req.headers().get_one("Authorization") {
            Some(token) => token,
            None => return Outcome::Failure((Status::BadRequest, ErrorKind::InvalidToken)),
        };

        let _token_data = match decode::<UserClaim>(
            token,
            &DecodingKey::from_secret(JWT_CONFIG.secret.as_bytes()),
            &VALIDATION,
        ) {
            Ok(data) => data,
            Err(err) => {
                println!("{:?}", err);
                return Outcome::Failure((Status::BadRequest, err.into_kind()));
            }
        };

        let conn = if let request::Outcome::Success(conn) = req.guard::<Connection<'_, Db>>().await
        {
            conn.into_inner()
        } else {
            return Outcome::Failure((Status::BadRequest, ErrorKind::InvalidToken));
        };

        // Try to insert the user
        let orange = entity::users::ActiveModel {
            uid: sea_orm::ActiveValue::Set(_token_data.claims.sub.clone()),
            created_at: sea_orm::ActiveValue::Set(chrono::Utc::now().into()),
            ..Default::default()
        };


        // let user = match User::insert()
        // .on_conflict(OnConflict::column(entity::users::Column::Uid).do_nothing())
        //     .filter(.contains(&_token_data.claims.sub))
        //     .one(conn)
        //     .await
        // {
        //     Ok(user) => user,
        //     Err(_) => return Outcome::Failure((Status::BadRequest, ErrorKind::InvalidAlgorithmName)),
        // };

        Outcome::Success(_token_data.claims)
    }
}
