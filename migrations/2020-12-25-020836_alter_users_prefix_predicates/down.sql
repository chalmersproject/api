ALTER TABLE users
    RENAME COLUMN is_email_verified TO email_verified;

ALTER TABLE users
    RENAME COLUMN is_phone_verified TO phone_verified;
