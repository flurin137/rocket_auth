use std::{future::Future, pin::Pin};

use rocket::{
    http::{Cookie, CookieJar, Status},
    request::{FromRequest, Outcome},
    Request,
};

#[macro_use]
extern crate rocket;

const COOKIE_NAME: &str = "ASDF";

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/secret")]
fn secret(person: Person) -> String {
    format!("Secret Data {}", person.name)
}

#[get("/login")]
fn login(cookies: &CookieJar<'_>) -> &'static str {
    cookies.add(Cookie::new(COOKIE_NAME, "asdf"));

    "Logged in"
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> &'static str {
    cookies.remove(Cookie::named(COOKIE_NAME));

    "Logged out"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, login, logout, secret])
}

struct Person {
    name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Person {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.cookies().get(COOKIE_NAME) {
            Some(cookie) => Outcome::Success(Person {
                name: cookie.value().to_string(),
            }),
            None => Outcome::Failure((Status::Forbidden, "What are you doing here".to_string())),
        }
    }
}
