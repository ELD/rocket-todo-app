use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use diesel::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;

type ConnectionPool = ::r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(db_url: &str) -> ConnectionPool {
    let manager = ConnectionManager::new(db_url);
    Pool::builder().build(manager).expect("database pool")
}

pub struct DbConn(pub ::r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<ConnectionPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
