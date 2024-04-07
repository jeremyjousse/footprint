use crate::{
    adapter::primary::tauri_command::consultation_type::consultation_type_dto::ConsultationTypeDto,
    domain::entity::consultation_type::ConsultationType, infrastructure::type_alias::Result,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

use crate::adapter::secondary::sqlite::consultation_type::consultation_type_repository::find;

pub fn consultation_type_list_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<ConsultationType>> {
    find(connection)
}
