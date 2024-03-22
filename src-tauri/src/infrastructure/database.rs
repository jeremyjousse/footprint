use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use tauri::AppHandle;

use crate::infrastructure::{path::global_db_path, type_alias::Result};

pub fn create_database_connection_pool(
    app_handle: &AppHandle,
) -> Result<Pool<ConnectionManager<SqliteConnection>>> {
    info!("loading DB from {:?}", global_db_path(app_handle)?);
    let manager =
        ConnectionManager::<SqliteConnection>::new(global_db_path(app_handle)?.to_str().unwrap());

    let pool = Pool::builder()
        .max_size(8) // TODO change to a variable
        .test_on_check_out(true)
        .build(manager)?;

    const MIGRATION: EmbeddedMigrations = embed_migrations!("./diesel/migrations");
    let _ = pool
        .get()
        .unwrap()
        .run_pending_migrations(MIGRATION)
        .unwrap();

    Ok(pool)
}
