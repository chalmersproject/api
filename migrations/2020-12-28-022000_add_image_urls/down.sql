ALTER TABLE users
    DROP COLUMN image_url;

ALTER TABLE shelters
    DROP COLUMN image_url;

ALTER TABLE shelters
    RENAME COLUMN website_url to website;
