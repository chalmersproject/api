ALTER TABLE users
    ADD COLUMN image_url TEXT;

ALTER TABLE shelters
    ADD COLUMN image_url TEXT;

ALTER TABLE shelters
    RENAME COLUMN website TO website_url;
