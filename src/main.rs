mod auth;
mod guards;
mod user;
mod hashing;

use crate::user::{Database, User};
use rocket::State;
use std::sync::Mutex;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    "Hello, world!".to_owned()
}

#[get("/secret")]
fn secret(database: &State<Mutex<Database>>, person: User) -> String {
    if let Ok(database) = database.lock() {
        return format!("Data {:?}", database.users);
    }

    format!("Secret Data {}", person.username)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(Database { users: vec![] }))
        .mount(
            "/",
            routes![index, secret, auth::login, auth::logout, auth::register],
        )
}
