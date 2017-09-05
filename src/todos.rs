use super::models::{Todo, NewTodo};
use super::schema::todo;
use super::schema::todo::dsl::todo as todo_dsl;
use super::db::DbConn;

use diesel::*;
use rocket_contrib::Json;

use rocket::response::status;

#[get("/")]
pub fn all(conn: DbConn) -> Json<Vec<Todo>> {
    Json(Todo::all(&*conn))
}

#[get("/<todo_id>")]
pub fn get(todo_id: i32, conn: DbConn) -> Json<Option<Todo>> {
    let todo = todo_dsl.find(todo_id).first(&*conn).ok();
    Json(todo)
}

#[post("/", format = "application/json", data = "<new_todo>")]
pub fn new(conn: DbConn, new_todo: Json<NewTodo>) -> status::Created<()> {
    let todo: Todo = insert_into(todo::table)
        .values(&new_todo.into_inner())
        .get_result(&*conn)
        .expect("Error saving new todo");

    status::Created(format!("localhost:8080/todos/{}", todo.id), None)
}
