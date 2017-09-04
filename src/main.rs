#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;

extern crate rocket;
extern crate todo_app;

use todo_app::todos;
use todo_app::db::init_pool;

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![todo_app::index])
        .mount("/todos", routes![todos::all, todos::new])
        .launch();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    fn bootstrap_rocket() -> rocket::Rocket {
        rocket::ignite()
            .mount("/", routes![todo_app::index])
            .mount("/todos", routes![todos::new])
    }

    #[test]
    fn hello_world_works() {
        let rocket = bootstrap_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
        let req = client.get("/");
        let mut response = req.dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }

    #[test]
    fn can_create_a_new_todo() {
        let rocket = bootstrap_rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
        let req = client.post("/todos")
            .header(ContentType::JSON)
            .body(r#"{ "body": "sample todo", "completed": false }"#);
        let mut response = req.dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let body = response.body().unwrap().into_string().unwrap();
        assert!(body.contains("sample todo"));
        assert!(body.contains(r#""completed":false"#));
    }
}
