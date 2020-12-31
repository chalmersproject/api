CREATE TABLE signals (
    id         UUID        PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    slug       TEXT        UNIQUE NOT NULL,
    name       TEXT        NOT NULL,
    shelter_id UUID        REFERENCES shelters(id) NOT NULL,
    measure    TEXT        NOT NULL,
    secret     TEXT        UNIQUE NOT NULL
);

ALTER TABLE shelter_measurements
    ADD COLUMN signal_id UUID NOT NULL REFERENCES signals(id);
