CREATE TABLE IF NOT EXISTS patients (
    id TEXT NOT NULL PRIMARY KEY,
    birthdate TIMESTAMP, -- todo update to SQLite date
    city TEXT,
    country TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    diabetic BOOLEAN NOT NULL, 
    email TEXT,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    long_duration_disease BOOLEAN NOT NULL,
    mobile_phone TEXT,
    national_insurance_number TEXT,
    notes TEXT NOT NULL,
    phone TEXT,
    postal_code TEXT,
    profession TEXT,
    street TEXT,
    title TEXT,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_personal_name ON patients(first_name, last_name);
CREATE INDEX idx_created_at ON patients(created_at);