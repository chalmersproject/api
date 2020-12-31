table! {
    shelter_measurements (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        shelter_id -> Uuid,
        occupied_spots -> Int4,
        occupied_beds -> Int4,
        total_spots -> Int4,
        total_beds -> Int4,
        signal_id -> Uuid,
    }
}

table! {
    shelters (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        slug -> Text,
        name -> Text,
        about -> Nullable<Text>,
        email -> Nullable<Text>,
        phone -> Text,
        website_url -> Nullable<Text>,
        address -> Jsonb,
        location -> Jsonb,
        total_spots -> Int4,
        total_beds -> Int4,
        food -> Text,
        tags -> Array<Text>,
        image_url -> Nullable<Text>,
        occupied_spots -> Nullable<Int4>,
        occupied_beds -> Nullable<Int4>,
    }
}

table! {
    signals (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        slug -> Text,
        name -> Text,
        shelter_id -> Uuid,
        measure -> Text,
        secret -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        firebase_id -> Text,
        slug -> Text,
        first_name -> Text,
        last_name -> Text,
        about -> Nullable<Text>,
        email -> Nullable<Text>,
        is_email_verified -> Bool,
        phone -> Nullable<Text>,
        is_phone_verified -> Bool,
        is_admin -> Bool,
        image_url -> Nullable<Text>,
    }
}

joinable!(shelter_measurements -> shelters (shelter_id));
joinable!(shelter_measurements -> signals (signal_id));
joinable!(signals -> shelters (shelter_id));

allow_tables_to_appear_in_same_query!(
    shelter_measurements,
    shelters,
    signals,
    users,
);
