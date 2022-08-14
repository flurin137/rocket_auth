use rocket::form::Form;
use rocket::http::Status;
use rocket::{
    http::{Cookie, CookieJar},
    State,
};
use std::sync::Mutex;

use crate::user::{Database, User};

pub const USER_ID_COOKIE_NAME: &str = "USER_ID";

fn database_error() -> (Status, String) {
    (Status::InternalServerError, "Database Error".to_string())
}

#[derive(FromForm)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[post("/login", data = "<credentials>")]
pub fn login(
    credentials: Form<Credentials>,
    cookies: &CookieJar<'_>,
    database: &State<Mutex<Database>>,
) -> (Status, String) {
    if let Ok(database) = database.lock() {
        if cookies.get_private(USER_ID_COOKIE_NAME).is_some() {
            return (Status::BadRequest, "User is already logged in".to_string());
        }

        if let Some(user) = database
            .users
            .iter()
            .find(|d| d.is_user_with_credentials(&credentials))
        {
            cookies.add_private(Cookie::new(USER_ID_COOKIE_NAME, user.user_id().to_string()));
            return (Status::Ok, format!("Logged in {}", user.username));
        }
        return (Status::Unauthorized, "Unable to log in".to_string());
    }
    database_error()
}

#[post("/register", data = "<credentials>")]
pub fn register(
    credentials: Form<Credentials>,
    cookies: &CookieJar<'_>,
    database: &State<Mutex<Database>>,
) -> (Status, String) {
    if let Ok(mut database) = database.lock() {
        if let Some(user) = database
            .users
            .iter()
            .find(|d| d.username == credentials.username)
        {
            return (
                Status::BadRequest,
                format!("User with name {} already registered", user.username),
            );
        }

        let user = User::from_credentials(&credentials);
        cookies.add_private(Cookie::new(USER_ID_COOKIE_NAME, user.user_id().to_string()));
        database.users.push(user);

        return (
            Status::Ok,
            format!("Registered user {}", credentials.username),
        );
    }

    database_error()
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> (Status, String) {
    if let Some(cookie) = cookies.get_private(USER_ID_COOKIE_NAME) {
        cookies.remove_private(cookie);
        return (Status::Ok, "Logged out".to_owned());
    }

    (Status::BadRequest, "Logged out".to_owned())
}
