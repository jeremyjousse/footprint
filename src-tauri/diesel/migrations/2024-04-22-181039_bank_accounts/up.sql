CREATE table IF NOT EXISTS bank_accounts (
    id TEXT NOT NULL PRIMARY KEY,
    account_number TEXT NOT NULL UNIQUE,
    bank_name TEXT NOT NULL,
    bank_cheque_deposit_number INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
