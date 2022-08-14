use std::sync::Mutex;

use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::{request::FromRequest, request::Outcome, Request, State};

use crate::auth::USER_ID_COOKIE_NAME;
use crate::user::{Database, User};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let database = req.guard::<&State<Mutex<Database>>>().await;

        if let Success(database) = database {
            if let Ok(database) = database.lock() {
                if let Some(cookie) = req.cookies().get_private(USER_ID_COOKIE_NAME) {
                    let user_id = cookie.value().to_string();
                    if let Some(user) = database
                        .users
                        .iter()
                        .find(|d| d.user_id().to_string() == user_id)
                    {
                        return Success(user.clone());
                    }
                }
            }
        };
        Failure((Status::BadRequest, ()))
    }
}
