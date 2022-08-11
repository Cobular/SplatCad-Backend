
use crate::guards::jwt::User;

#[get("/user/id")]
pub fn user_id(user: User) -> String {
    user.sub
}