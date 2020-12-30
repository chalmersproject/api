ALTER TABLE shelter_occupancies
    RENAME TO shelter_measurements;

ALTER TABLE shelter_measurements
    ADD COLUMN total_spots INT NOT NULL,
    ADD COLUMN total_beds INT NOT NULL;

ALTER TABLE shelters
    RENAME COLUMN spots TO total_spots;

ALTER TABLE shelters
    RENAME COLUMN beds TO total_beds;

ALTER TABLE shelters
    ADD COLUMN occupied_spots INT,
    ADD COLUMN occupied_beds INT;
