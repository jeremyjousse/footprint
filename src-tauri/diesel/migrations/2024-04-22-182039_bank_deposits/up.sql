CREATE table IF NOT EXISTS bank_deposits (
    id TEXT NOT NULL PRIMARY KEY,
    bank_account_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deposit_number INTEGER  NOT NULL,
    amount DECIMAL(4, 2) NOT NULL,
    deposit_date DATE  NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (bank_account_id) REFERENCES bank_accounts(id)
);
