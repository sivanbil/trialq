diesel::table! {
    tools (id) {
        id -> Integer,
        name -> Text,
        link_url -> Text,
        sort_value -> Integer,
        is_valid -> Bool,
    }
}