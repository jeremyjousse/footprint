CREATE TABLE IF NOT EXISTS consultations (
    id TEXT NOT NULL PRIMARY KEY,
    appointment_date_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    consultation_type TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    location TEXT NOT NULL,
    note TEXT,
    patient_id INTEGER,
    price DECIMAL(4, 2) NOT NULL,
    -- TODO do we need a status
    status TEXT  NOT NULL DEFAULT 'done',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);