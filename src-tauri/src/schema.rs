// @generated automatically by Diesel CLI.

diesel::table! {
    project_data_clean_progress (id) {
        id -> Nullable<Integer>,
        study -> Text,
        site -> Text,
        subject -> Text,
        folder_name -> Text,
        page -> Text,
        entered -> Integer,
        verify_required -> Integer,
        create_time -> Text,
        update_time -> Nullable<Text>,
    }
}

diesel::table! {
    project_missing_page (id) {
        id -> Nullable<Integer>,
        project_name -> Text,
        site_name -> Text,
        site_number -> Text,
        subject_id -> Text,
        instance_name -> Text,
        data_page_name -> Text,
        days_of_missing_pages -> Integer,
        create_time -> Text,
        update_time -> Nullable<Text>,
    }
}

diesel::table! {
    project_query_detail (id) {
        id -> Nullable<Integer>,
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
        qry_response_date -> Nullable<Text>,
        qry_response_date_localized -> Nullable<Text>,
        qry_response_user -> Text,
        qry_answer -> Text,
        qry_status -> Text,
    }
}

diesel::table! {
    project_report (id) {
        id -> Nullable<Integer>,
        project_number -> Text,
        report_number -> Text,
        state -> Integer,
        create_time -> Text,
        update_time -> Text,
    }
}

diesel::table! {
    project_report_data (id) {
        id -> Nullable<Integer>,
        site -> Text,
        site_name -> Text,
        cra -> Text,
        enrollment -> Text,
        pages -> Text,
        pages_entered -> Text,
        missing_pages -> Text,
        sdv_backlog -> Text,
        edc_status_comment -> Nullable<Text>,
        plan_rmv -> Nullable<Text>,
        latest_rmv -> Nullable<Text>,
    }
}

diesel::table! {
    project_report_extend_data (id) {
        id -> Nullable<Integer>,
        project_report_data_id -> Integer,
        data_group -> Text,
        percent_pages_entered -> Text,
        percent_pages_sdved -> Text,
        answered_query -> Text,
        opened_query -> Text,
        oq_gt7 -> Integer,
        oq_gt14 -> Integer,
        op_gt21 -> Integer,
        op_gt30 -> Integer,
    }
}

diesel::table! {
    project_report_source (id) {
        id -> Nullable<Integer>,
        project_number -> Text,
        report_number -> Text,
        source_file_name -> Text,
        source_file_type -> Text,
        create_time -> Text,
        update_time -> Text,
    }
}

diesel::table! {
    projects (id) {
        id -> Nullable<Integer>,
        project_name -> Text,
        description -> Text,
        create_time -> Text,
        update_time -> Nullable<Text>,
    }
}

diesel::table! {
    tools (id) {
        id -> Nullable<Integer>,
        name -> Text,
        link_url -> Text,
        sort_value -> Integer,
        is_valid -> Bool,
    }
}

diesel::table! {
    user_task (id) {
        id -> Nullable<Integer>,
        task_no -> Text,
        related_id -> Integer,
        task_category -> Integer,
        state -> Integer,
        create_time -> Text,
        update_time -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        email -> Text,
        password -> Text,
        random -> Text,
        active_code -> Text,
        validate_date -> Text,
        expire_date -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    project_data_clean_progress,
    project_missing_page,
    project_query_detail,
    project_report,
    project_report_data,
    project_report_extend_data,
    project_report_source,
    projects,
    tools,
    user_task,
    users,
);
