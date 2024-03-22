CREATE TABLE IF NOT EXISTS patients (
    id TEXT PRIMARY KEY,
    birthdate TEXT,
    city TEXT,
    created_at TEXT NOT NULL,
    email TEXT,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    mobile_phone TEXT,
    note TEXT NOT NULL,
    phone TEXT,
    title TEXT,
    country TEXT,
    postal_code TEXT,
    street TEXT,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_personal_name ON reference_tracks(first_name, last_name);
CREATE INDEX idx_created_at ON reference_tracks(created_at);