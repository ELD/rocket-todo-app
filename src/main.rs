#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate todo_app;
extern crate dotenv;

use std::env;

use dotenv::dotenv;

use todo_app::bootstrap_rocket;

fn main() {
    dotenv().ok();
    let rocket = bootstrap_rocket(env::var("DATABASE_URL").expect("Database URL").as_ref());

    rocket.launch();
}
