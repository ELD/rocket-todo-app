#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2;

pub mod todos;
pub mod schema;
pub mod models;
pub mod db;

use rocket::Rocket;

use db::init_pool;

#[get("/")]
pub fn index_route() -> &'static str {
    "Hello, Rocket on Docker!"
}

pub fn bootstrap_rocket(connection_string: &str) -> Rocket {
    rocket::ignite()
        .manage(init_pool(connection_string))
        .mount("/", routes![index_route])
        .mount("/todo", todos::routes())
}
