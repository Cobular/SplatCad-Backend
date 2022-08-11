use jsonwebtoken::errors::ErrorKind;
use migration::DbErr;

#[derive(Debug)]
pub enum MyError {
    JwtError(ErrorKind),
    DbErr(DbErr),
}

impl From<ErrorKind> for MyError {
    fn from(err: ErrorKind) -> Self {
        MyError::JwtError(err)
    }
}

impl From<DbErr> for MyError {
    fn from(err: DbErr) -> Self {
        MyError::DbErr(err)
    }
}
