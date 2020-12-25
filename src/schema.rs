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
