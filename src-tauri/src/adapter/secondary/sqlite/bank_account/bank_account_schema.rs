use diesel::table;

table! {
    bank_accounts (id) {
        created_at -> Timestamp,
        id -> Text,
        account_number -> Text,
        bank_name -> Text,
        bank_cheque_deposit_number -> Integer,
        updated_at -> Timestamp,
    }
}
