use rocket::form::Form;
use rocket::{
    http::{Cookie, CookieJar},
    State,
};
use std::sync::Mutex;

use crate::models::{Database, User};

pub const USER_ID_COOKIE_NAME: &str = "USER_ID";

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
) -> String {
    if let Ok(database) = database.lock() {
        let credentials = credentials.into_inner();
        if let Some(user) = database
            .users
            .iter()
            .find(|d| d.is_user_with_credentials(&credentials))
        {
            cookies.add_private(Cookie::new(USER_ID_COOKIE_NAME, user.username.clone()));
            return format!("Logged in {}", user.username);
        }
    }
    "Unable to log in".to_string()
}

#[post("/register", data = "<credentials>")]
pub fn register(
    credentials: Form<Credentials>,
    cookies: &CookieJar<'_>,
    database: &State<Mutex<Database>>,
) -> String {
    if let Ok(mut database) = database.lock() {
        if let Some(user) = database
            .users
            .iter()
            .find(|d| d.username == credentials.username)
        {
            return format!("User with name {} already registered", user.username);
        }

        let user = User::from_credentials(&credentials);
        database.users.push(user);

        cookies.add_private(Cookie::new(
            USER_ID_COOKIE_NAME,
            credentials.username.clone(),
        ));
    }

    "Registered".to_owned()
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> String {
    cookies.remove_private(Cookie::named(USER_ID_COOKIE_NAME));

    "Logged out".to_owned()
}
