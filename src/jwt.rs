use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, Validation};
use once_cell::sync::Lazy;
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    Config, Request,
};
use serde::{Deserialize, Serialize};

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
pub struct User {
    pub sub: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ErrorKind;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = match req.headers().get_one("Authorization") {
            Some(token) => token,
            None => return Outcome::Failure((Status::BadRequest, ErrorKind::InvalidToken)),
        };

        let _token_data = match decode::<User>(
            token,
            &DecodingKey::from_secret(JWT_CONFIG.secret.as_bytes()),
            &VALIDATION,
        ) {
            Ok(data) => data,
            Err(err) => {
                println!("{:?}", err);
                return Outcome::Failure((Status::BadRequest, err.into_kind()));
            },
        };

        Outcome::Success(_token_data.claims)
    }
}
