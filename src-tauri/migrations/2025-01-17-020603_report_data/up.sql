-- Your SQL goes here
create table project_report_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    site TEXT NOT NULL UNIQUE,
    site_name TEXT NOT NULL,
    cra TEXT NOT NULL,
    pages INTEGER NOT NULL,
    pages_entered INTEGER NOT NULL,
    missing_pages INTEGER NOT NULL,
    md_gt7 INTEGER NOT NULL,
    md_gt14 INTEGER NOT NULL,
    sdv_backlog INTEGER NOT NULL,
    edc_status_comment TEXT,
    percent_pages_entered TEXT NOT NULL,
    percent_pages_sdved TEXT NOT NULL,
    answered_query INTEGER NOT NULL,
    opened_query INTEGER NOT NULL,
    op_gt7 INTEGER NOT NULL,
    op_gt14 INTEGER NOT NULL,
    op_gt21 INTEGER NOT NULL,
    op_gt30 INTEGER NOT NULL,
    report_number TEXT NOT NULL
);