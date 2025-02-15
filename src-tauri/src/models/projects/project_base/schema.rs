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

diesel::table! {
    project_site (id) {
        id -> Integer,
        project_name -> Text,
        site_number -> Text,
        site_name -> Text,
        site_cra -> Text,
    }
}
