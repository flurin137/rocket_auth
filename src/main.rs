use std::sync::Mutex;

use rocket::outcome::Outcome::*;
use rocket::{
    http::{Cookie, CookieJar, Status},
    request::{FromRequest, Outcome},
    Request, State,
};

#[macro_use]
extern crate rocket;

const COOKIE_NAME: &str = "ASDF";

#[get("/")]
fn index() -> String {
    "Hello, world!".to_owned()
}

#[get("/secret")]
fn secret(database: &State<Mutex<Database>>, person: User) -> String {
    if let Ok(database) = database.lock() {
        return format!("Data {:?}", database.users);
    }

    format!("Secret Data {}", person.name)
}

#[get("/login")]
fn login(cookies: &CookieJar<'_>) -> String {
    let user = "Fx";
    cookies.add_private(Cookie::new(COOKIE_NAME, user));
    format!("Logged in {}", user)
}

#[get("/register")]
fn register(cookies: &CookieJar<'_>, database: &State<Mutex<Database>>) -> String {
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
fn logout(cookies: &CookieJar<'_>) -> String {
    cookies.remove_private(Cookie::named(COOKIE_NAME));

    "Logged out".to_owned()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(Database { users: vec![] }))
        .mount("/", routes![index, login, logout, register, secret])
}

struct Database {
    users: Vec<User>,
}

#[derive(Debug, Clone)]
struct User {
    name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let database = req.guard::<&State<Mutex<Database>>>().await;

        if let Success(database) = database {
            if let Ok(database) = database.lock() {
                if let Some(cookie) = req.cookies().get_private(COOKIE_NAME) {
                    let name = cookie.value().to_string();
                    if let Some(user) = database.users.iter().find(|d| d.name == name) {
                        return Success(user.clone());
                    }
                }
            }
        };
        Failure((Status::BadRequest, ()))
    }
}
