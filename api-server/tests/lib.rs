#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate rocket;
extern crate libtodo;

use std::io::Error;

use diesel::prelude::*;

use rocket::Rocket;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

use todo_app::db::{DbConn, init_pool};
use todo_app::todos;

pub fn bootstrap_rocket(connection_string: &str) -> (Rocket, Option<DbConn>) {
    let pool = init_pool(connection_string);

    let conn = if cfg!(test) {
        Some(DbConn(pool.get().expect("Database connection")))
    } else {
        None
    };

    let rocket = rocket::ignite()
        .manage(pool)
        .mount("/todos", routes![todos::all, todos::new]);

    (rocket, conn)
}

#[test]
#[ignore]
fn can_list_all_todos() {
    let (rocket, pool) = bootstrap_rocket("postgres://todo:todo@localhost/todo_test");

    match pool {
        Some(_) => println!("Got a DB connection"),
        None => println!("Don't have a DB Connection"),
    }

    let client = Client::new(rocket).expect("Valid rocket instance");
    let req = client.get("/todos");
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body().unwrap().into_string().unwrap();
    println!("{}", body);
    assert!(body.contains("sample todo"));
    assert!(body.contains("another sample todo"));
}

#[test]
fn can_create_a_new_todo() {
    let (rocket, pool) = bootstrap_rocket("postgres://todo:todo@localhost/todo_test");

    let connection = match pool {
        Some(connection) => connection,
        None => panic!("Failed to get database connection! Aborting...")
    };

    (*connection).test_transaction::<_, Error, _>(|| {
        let client = Client::new(rocket).expect("valid rocket instance");
        let req = client.post("/todos")
            .header(ContentType::JSON)
            .body(r#"{ "description": "sample todo", "completed": false }"#);
        let response = req.dispatch();

        assert_eq!(response.status(), Status::Created);

        Ok(())
    });
}
