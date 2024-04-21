CREATE TABLE IF NOT EXISTS consultations (
    id TEXT NOT NULL PRIMARY KEY,
    appointment_date_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    consultation_type TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    location TEXT NOT NULL,
    note TEXT,
    patient_id TEXT NOT NULL,
    price DECIMAL(4, 2) NOT NULL,
    status TEXT NOT NULL DEFAULT 'Done',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(patient_id) REFERENCES patients(id)
);