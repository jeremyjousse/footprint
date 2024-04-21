use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection,
};

use crate::{
    domain::{
        entity::payment::Payment,
        value_object::{listing_sort::ListingSort, payment_list_filter::PaymentListFilter},
    },
    infrastructure::type_alias::Result,
};

use super::{
    payment_model::DbPayment,
    payment_schema::payments::{
        self, amount, consultation_id, created_at, id, payed_at, payment_method, status,
    },
};

pub fn add(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    payment: Payment,
) -> Result<Payment> {
    let rows_inserted = diesel::insert_into(payments::table)
        .values(DbPayment::from(payment))
        .get_result::<DbPayment>(connection)
        .unwrap();

    Ok(rows_inserted.into())
}

pub fn find(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    filter: PaymentListFilter,
    sort: ListingSort,
) -> Result<Vec<Payment>> {
    let mut payments_query = payments::table.into_boxed();

    if filter.consultation_id.is_some() {
        payments_query = payments_query.filter(consultation_id.eq(filter.consultation_id.unwrap()));
    }

    payments_query = match sort {
        ListingSort { by, order } if by == *"payed_at" && order == *"asc" => {
            payments_query.order(payed_at.asc())
        }
        ListingSort { by, order } if by == *"payed_at" && order == *"desc" => {
            payments_query.order(payed_at.desc())
        }
        _ => payments_query.order(created_at.desc()),
    };

    let db_payments: Vec<DbPayment> = payments_query.load::<DbPayment>(connection)?;

    let payments = db_payments.into_iter().map(Payment::from).collect();

    Ok(payments)
}

pub fn update(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    payment: Payment,
) -> Result<Payment> {
    let db_payment = DbPayment::from(payment);
    let db_payment = diesel::update(payments::table)
        .filter(id.eq(db_payment.id))
        .set((
            amount.eq(db_payment.amount),
            payed_at.eq(db_payment.payed_at),
            payment_method.eq(db_payment.payment_method),
            status.eq(db_payment.status),
        ))
        .get_result::<DbPayment>(connection)
        .expect("Error updating payment");
    Ok(Payment::from(db_payment))
}
