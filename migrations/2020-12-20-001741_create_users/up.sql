CREATE TABLE users (
    id             UUID        PRIMARY KEY,
    created_at     TIMESTAMPTZ NOT NULL,
    updated_at     TIMESTAMPTZ NOT NULL,
    firebase_id    TEXT        UNIQUE NOT NULL,
    slug           TEXT        UNIQUE NOT NULL,
    first_name     TEXT        NOT NULL,
    last_name      TEXT        NOT NULL,
    about          TEXT,
    email          TEXT        UNIQUE,
    email_verified BOOLEAN     NOT NULL,
    phone          TEXT        UNIQUE,
    phone_verified BOOLEAN     NOT NULL,
    is_admin       BOOLEAN     NOT NULL
);
