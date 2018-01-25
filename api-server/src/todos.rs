use super::models::{Todo, NewTodo};
use super::schema::todo;
use super::schema::todo::dsl::todo as todo_dsl;
use super::db::DbConn;

use diesel::*;
use rocket_contrib::Json;

use rocket::Route;
use rocket::response::{content, status};
use rocket::http::Status;

#[get("/")]
pub fn all(conn: DbConn) -> Json<Vec<Todo>> {
    Json(Todo::all(&*conn))
}

#[get("/<todo_id>")]
pub fn get<'a>(todo_id: i32, conn: DbConn) -> Result<status::Custom<Json<Todo>>, status::NotFound<content::Json<&'a str>>> {
    let todo = todo_dsl.find(todo_id).first(&*conn);

    match todo {
        Ok(todo) => Ok(status::Custom(Status::Ok, Json(todo))),
        Err(_) => Err(status::NotFound(content::Json("{'status':404,'reason':'resource not found'}"))),
    }
}

#[put("/<todo_id>", format = "application/json", data = "<todo_update>")]
pub fn update<'a>(todo_id: i32, todo_update: Json<NewTodo>, conn: DbConn) -> Result<(), status::NotFound<content::Json<&'a str>>> {
    ::diesel::update(todo_dsl.find(todo_id))
        .set(&todo_update.into_inner())
        .get_result::<Todo>(&*conn)
        .expect("Could not update todo");

    Ok(())
}

#[post("/", format = "application/json", data = "<new_todo>")]
pub fn new<'a>(conn: DbConn, new_todo: Json<NewTodo>) -> status::Created<&'a str> {
    let todo: Todo = insert_into(todo::table)
        .values(&new_todo.into_inner())
        .get_result(&*conn)
        .expect("Error saving new todo");

    status::Created(format!("localhost:8080/todo/{}", todo.id), None)
}

pub fn routes() -> Vec<Route> {
    routes![all, get, new, update]
}
