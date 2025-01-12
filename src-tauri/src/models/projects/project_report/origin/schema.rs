// schema.rs
table! {
    project_query_detail (id) {
        id -> Integer,
        study -> Text,
        site_name -> Text,
        study_environment_site -> Text,
        subject_name -> Text,
        folder -> Text,
        form -> Text,
        log_id -> Integer,
        qry_open_date -> Text,
        qry_open_date_localized -> Text,
        qry_open_by -> Text,
        query_text -> Text,
        marking_group_name -> Text,
        qry_response_date -> Text,
        qry_response_date_localized -> Text,
        qry_response_user -> Text,
        qry_answer -> Text,
        qry_status -> Text,
        report_number -> Text,
    }
}

// schema.rs
table! {
    project_data_clean_progress (id) {
        id -> Integer,
        study -> Text,
        site -> Text,
        subject -> Text,
        folder_name -> Text,
        page -> Text,
        entered -> Integer,
        verify_required -> Integer,
        create_time -> Text,
        update_time -> Text,
        report_number -> Text,
    }
}

// schema.rs
table! {
    project_missing_page (id) {
        id -> Integer,
        project_name -> Text,
        site_name -> Text,
        site_number -> Text,
        subject_id -> Text,
        instance_name -> Text,
        data_page_name -> Text,
        days_of_missing_pages -> Integer,
        create_time -> Text,
        update_time -> Text,
        report_number -> Text,
    }
}
