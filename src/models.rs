#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}