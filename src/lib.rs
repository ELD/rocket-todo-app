#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;

pub mod todos;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
