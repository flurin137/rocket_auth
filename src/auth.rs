use rocket::{
    data,
    http::{Cookie, CookieJar},
    State,
};
use std::sync::Mutex;

use crate::models::{Database, User};

pub const COOKIE_NAME: &str = "ASDF";

#[get("/login")]
pub fn login(cookies: &CookieJar<'_>, database: &State<Mutex<Database>>) -> String {
    if let Ok(database) = database.lock() {
        let user_name = "Fx";
        if let Some(user) = database.users.iter().find(|d| d.name == user_name) {
            cookies.add_private(Cookie::new(COOKIE_NAME, user.name.clone()));
            return format!("Logged in {}", user_name);
        }
    }
    "Unable to log in".to_string()
}

#[get("/register")]
pub fn register(cookies: &CookieJar<'_>, database: &State<Mutex<Database>>) -> String {
    let user = "Fx";
    cookies.add_private(Cookie::new(COOKIE_NAME, user));

    if let Ok(mut database) = database.lock() {
        database.users.push(User {
            name: user.to_string(),
        });
    }

    "Registered".to_owned()
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> String {
    cookies.remove_private(Cookie::named(COOKIE_NAME));

    "Logged out".to_owned()
}
