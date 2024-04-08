use crate::{
    domain::{
        entity::consultation::Consultation,
        value_object::{
            consultation_list_filter::ConsultationListFilter, listing_sort::ListingSort,
        },
    },
    infrastructure::type_alias::Result,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

use crate::adapter::secondary::sqlite::consultation::consultation_repository::find;

pub fn consultation_list_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    filter: ConsultationListFilter,
    sort: ListingSort,
) -> Result<Vec<Consultation>> {
    find(connection, filter, sort)
}
