use diesel::{
    r2d2::{ConnectionManager, Pool},
    sqlite::SqliteConnection,
};

pub struct DbState {
    pub global: Pool<ConnectionManager<SqliteConnection>>,
}
