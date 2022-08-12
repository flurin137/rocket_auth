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
        if let Some(user) = database
            .users
            .iter()
            .find(|d| d.name == credentials.username && d.password == credentials.password)
        {
            cookies.add_private(Cookie::new(USER_ID_COOKIE_NAME, user.name.clone()));
            return format!("Logged in {}", user.name);
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
            .find(|d| d.name == credentials.username)
        {
            return format!("User with name {} already registered", user.name);
        }

        database.users.push(User {
            name: credentials.username.clone(),
            password: credentials.password.clone(),
        });
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
