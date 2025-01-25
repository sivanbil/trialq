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
        pages -> Integer,
        pages_entered -> Integer,
        missing_pages -> Integer,
        md_gt7 -> Integer,
        md_gt14 -> Integer,
        sdv_backlog -> Integer,
        edc_status_comment -> Text,
        percent_pages_entered -> Text,
        percent_pages_sdved -> Text,
        answered_query -> Integer,
        opened_query -> Integer,
        op_gt7 -> Integer,
        op_gt14 -> Integer,
        op_gt21 -> Integer,
        op_gt30 -> Integer,
        report_number -> Text
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