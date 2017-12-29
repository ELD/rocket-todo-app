//infer_schema!("dotenv:DATABASE_URL");
table!{
    todo (id) {
        id -> Integer,
        description -> Text,
        completed -> Bool,
    }
}