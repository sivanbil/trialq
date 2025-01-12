// schema.rs
use diesel::prelude::*;
table! {
    project_report (id) {
        id -> Integer,
        project_number -> Text,
        report_number -> Text,
        state -> Integer,
        create_time -> Text,
        update_time -> Text,
    }
}

// schema.rs
table! {
    project_report_data (id) {
        id -> Integer,
        site -> Text,
        site_name -> Text,
        cra -> Text,
        enrollment -> Text,
        pages -> Text,
        pages_entered -> Text,
        missing_pages -> Text,
        sdv_backlog -> Text,
        edc_status_comment -> Text,
        plan_rmv -> Text,
        latest_rmv -> Text,
    }
}

// schema.rs
table! {
    project_report_extend_data (id) {
        id -> Integer,
        project_report_data_id -> Integer,
        data_group -> Text,
        percent_pages_entered -> Text,
        percent_pages_solved -> Text,
        answered_query -> Text,
        opened_query -> Text,
        od_gt7 -> Integer,
        od_gt14 -> Integer,
        op_gt21 -> Integer,
        op_gt30 -> Integer,
    }
}
table! {
    project_report_source (id) {
        id -> Integer,
        project_number -> Text,
        report_number -> Text,
        source_file_name -> Text,
        source_file_type -> Text,
        create_time -> Text,
        update_time -> Text,
    }
}