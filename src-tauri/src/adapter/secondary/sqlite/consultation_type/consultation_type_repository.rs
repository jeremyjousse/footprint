use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
};

use crate::{
    adapter::primary::tauri_command::consultation_type,
    domain::entity::consultation_type::ConsultationType, infrastructure::type_alias::Result,
};

use super::{
    consultation_type_model::DbConsultationType,
    consultation_type_schema::consultation_types::{self, name},
};

pub fn add(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    consultation_type: ConsultationType,
) -> Result<ConsultationType> {
    let rows_inserted = diesel::insert_into(consultation_types::table)
        .values(&DbConsultationType::from(consultation_type))
        .get_result::<DbConsultationType>(connection)
        .unwrap();

    Ok(ConsultationType::from(rows_inserted))
}

pub fn find(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<ConsultationType>> {
    let db_consultation_types: Vec<DbConsultationType> = consultation_types::table
        .order(name.asc())
        .load::<DbConsultationType>(connection)?;

    let consultation_types = db_consultation_types
        .into_iter()
        .map(ConsultationType::from)
        .collect();

    Ok(consultation_types)
}
