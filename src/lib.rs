#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2_diesel;
extern crate r2d2;

pub mod todos;
pub mod schema;
pub mod models;
pub mod db;

use rocket::Rocket;

use db::init_pool;

pub fn bootstrap_rocket(connection_string: &str) -> Rocket {
    rocket::ignite()
        .manage(init_pool(connection_string))
        .mount("/todos", routes![todos::all, todos::new])
}
