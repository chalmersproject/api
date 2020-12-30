ALTER TABLE shelter_measurements
    RENAME TO shelter_occupancies;

ALTER TABLE shelter_occupancies
    DROP COLUMN total_spots,
    DROP COLUMN total_beds;

ALTER TABLE shelters
    RENAME COLUMN total_spots TO spots;

ALTER TABLE shelters
    RENAME COLUMN total_beds TO beds;

ALTER TABLE shelters
    DROP COLUMN occupied_spots,
    DROP COLUMN occupied_beds;
