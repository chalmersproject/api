ALTER TABLE users
    RENAME COLUMN email_verified TO is_email_verified;

ALTER TABLE users
    RENAME COLUMN phone_verified TO is_phone_verified;
