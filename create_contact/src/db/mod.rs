use diesel::{Connection, prelude::PgConnection};

pub fn create_connection() -> PgConnection {
    let database_url = "postgres://postgres:local_password@localhost:5432/contact";

    PgConnection::establish(database_url).unwrap_or_else(|_| {
        panic!(
            "create_connection - Exception: error to connect to {}",
            database_url
        )
    })
}

