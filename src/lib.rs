#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel_codegen;

extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;

pub mod todos;
pub mod schema;
pub mod models;
pub mod db;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
