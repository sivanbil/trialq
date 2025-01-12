// schema.rs
diesel::table! {
    projects (id) {
        id -> Integer,
        project_name -> Text,
        description -> Text,
        create_time -> Text,
        update_time -> Text,
    }
}
