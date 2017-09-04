use std::env;
use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use diesel::mysql::MysqlConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::{Config, Pool};
use dotenv::dotenv;

type MysqlPool = ::r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> MysqlPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database_URL must be set.");
    let config = Config::default();
    let manager = ConnectionManager::new(database_url);
    Pool::new(config, manager).expect("DB Pool")
}

pub struct DbConn(pub ::r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<MysqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
