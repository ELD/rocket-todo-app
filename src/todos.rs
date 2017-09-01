use rocket_contrib::Json;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    body: String,
    completed: bool,
}

#[post("/", format = "application/json", data = "<todo>")]
pub fn new(todo: Json<Todo>) -> Option<Json<Todo>> {
    Some(todo)
}
