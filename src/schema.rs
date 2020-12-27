table! {
    shelter_occupancies (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        shelter_id -> Uuid,
        occupied_spots -> Int4,
        occupied_beds -> Int4,
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
        website -> Nullable<Text>,
        address -> Jsonb,
        location -> Jsonb,
        spots -> Int4,
        beds -> Int4,
        food -> Text,
        tags -> Array<Text>,
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
    }
}

joinable!(shelter_occupancies -> shelters (shelter_id));

allow_tables_to_appear_in_same_query!(shelter_occupancies, shelters, users,);
