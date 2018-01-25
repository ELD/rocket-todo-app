use diesel::prelude::*;

use schema::todo;
use schema::todo::dsl::{todo as all_todos};

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Insertable, Debug, Clone, AsChangeset)]
#[table_name = "todo"]
pub struct NewTodo {
    pub description: String,
    pub completed: bool,
}

impl Todo {
    pub fn all(conn: &PgConnection) -> Vec<Todo> {
        all_todos.order(todo::id.desc()).load::<Todo>(conn).unwrap()
    }
}
