CREATE TABLE shelters (
    id         UUID        PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    slug       TEXT        UNIQUE NOT NULL,
    name       TEXT        UNIQUE NOT NULL,
    about      TEXT,
    email      TEXT        UNIQUE,
    phone      TEXT        UNIQUE NOT NULL,
    website    TEXT,
    address    JSONB       NOT NULL,
    location   JSONB       NOT NULL,
    spots      INT         NOT NULL,
    beds       INT         NOT NULL,
    food       TEXT        NOT NULL,
    tags       TEXT[]      NOT NULL
);

CREATE TABLE shelter_occupancies (
    id             UUID        PRIMARY KEY,
    created_at     TIMESTAMPTZ NOT NULL,
    updated_at     TIMESTAMPTZ NOT NULL,
    shelter_id     UUID        NOT NULL REFERENCES shelters(id),
    occupied_spots INT         NOT NULL,
    occupied_beds  INT         NOT NULL
);
