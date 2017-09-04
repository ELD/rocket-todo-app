use super::models::Todo;
use super::schema::todo;
use super::schema::todo::dsl::*;
use super::db::DbConn;

use diesel::*;
use rocket_contrib::Json;


#[get("/")]
pub fn all(conn: DbConn) -> QueryResult<Json<Vec<Todo>>> {
    todo.order(todo::id.desc()).load::<Todo>(&*conn).map(|t| Json(t))
}

#[post("/", format = "application/json", data = "<new_todo>")]
pub fn new(new_todo: Json<Todo>) -> Option<Json<Todo>> {
    Some(new_todo)
}
