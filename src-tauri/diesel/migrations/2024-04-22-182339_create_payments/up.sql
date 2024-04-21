CREATE table IF NOT EXISTS payments (
    id TEXT NOT NULL PRIMARY KEY,
    amount DECIMAL(4, 2) NOT NULL,
    consultation_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    payed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    payment_method TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Payed',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(consultation_id) REFERENCES consultations(id)
);

CREATE table IF NOT EXISTS payment_bank_cheques (
    id TEXT NOT NULL PRIMARY KEY,
    account_owner TEXT NOT NULL,
    bank_deposit_id TEXT,
    bank_name TEXT NOT NULL,
    check_number TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(bank_deposit_id) REFERENCES bank_deposits(id)
);