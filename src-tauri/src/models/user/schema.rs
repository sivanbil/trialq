diesel::table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        random -> Text,
        active_code -> Text,
        validate_date -> Text,
        expire_date -> Text,
    }
}
