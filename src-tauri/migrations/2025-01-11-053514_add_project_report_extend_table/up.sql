-- Your SQL goes here
create table users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    random TEXT NOT NULL,
    active_code TEXT NOT NULL UNIQUE,
    validate_date TEXT NOT NULL ,
    expire_date TEXT NOT NULL,
    UNIQUE (email)
);

-- 常用工具
create table tools (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    link_url TEXT NOT NULL UNIQUE,
    sort_value INTEGER NOT NULL,
    is_valid BOOL NOT NULL ,
    UNIQUE  (link_url)
);
-- 负责项目
create table projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_name TEXT NOT NULL,
    description TEXT NOT NULL,
    create_time TEXT NOT NULL ,
    update_time TEXT ,
    UNIQUE (project_name)
);
create table project_site (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_name TEXT NOT NULL,
    site_number TEXT NOT NULL,
    site_name TEXT NOT NULL,
    site_cra TEXT NOT NULL
);
-- 创建一个名为 idx_project_name 的索引，该索引基于 project_site 表的 project_name 字段
CREATE INDEX idx_project_name ON project_site (project_name);

create table project_report (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_number TEXT NOT NULL,
    report_number TEXT NOT NULL UNIQUE,
    state INTEGER NOT NULL,
    create_time TEXT NOT NULL,
    update_time TEXT NOT NULL
);

-- 为 project_number 字段创建索引
CREATE INDEX idx_project_number ON project_report (project_number);

-- 为 report_number 字段创建索引
CREATE INDEX idx_report_number_1 ON project_report (report_number);

create table user_task (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_no TEXT NOT NULL UNIQUE,
    related_id INTEGER NOT NULL ,
    task_category INTEGER NOT NULL,
    state INTEGER NOT NULL,
    create_time TEXT NOT NULL,
    update_time TEXT NOT NULL
);

-- 源文件映射
create table project_report_source (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_number TEXT NOT NULL,
    report_number TEXT NOT NULL,
    source_file_name TEXT NOT NULL,
    source_file_type TEXT NOT NULL,
    create_time TEXT NOT NULL,
    update_time TEXT NOT NULL
);
CREATE INDEX idx_report_number_2 ON project_report_source (report_number);

-- 报告汇总表
create table project_report_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    site TEXT NOT NULL,
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
CREATE INDEX idx_report_number_3 ON project_report_data (report_number);

-- 项目missing_page数据
CREATE TABLE project_missing_page (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_name TEXT NOT NULL,
    site_name TEXT NOT NULL,
    site_number TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    instance_name TEXT NOT NULL,
    data_page_name TEXT NOT NULL,
    days_of_missing_pages INTEGER NOT NULL,
    create_time TEXT NOT NULL,
    update_time TEXT,
    report_number TEXT NOT NULL
);
CREATE INDEX idx_report_number_4 ON project_missing_page (report_number);

-- 项目query_detail数据--对应源数据
CREATE TABLE project_query_detail (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    study TEXT NOT NULL,
    site_name TEXT NOT NULL,
    study_environment_site TEXT NOT NULL,
    subject_name TEXT NOT NULL,
    folder TEXT NOT NULL,
    form TEXT NOT NULL,
    log_id INTEGER NOT NULL,
    qry_open_date TEXT NOT NULL,
    qry_open_date_localized TEXT NOT NULL,
    qry_open_by TEXT NOT NULL,
    query_text TEXT NOT NULL,
    marking_group_name TEXT NOT NULL,
    qry_response_date TEXT,
    qry_response_date_localized TEXT,
    qry_response_user TEXT NOT NULL,
    qry_answer TEXT NOT NULL,
    qry_status TEXT NOT NULL,
    create_time TEXT NOT NULL,
    update_time TEXT,
    report_number TEXT NOT NULL
);
CREATE INDEX idx_report_number_5 ON project_query_detail (report_number);

-- 项目的数据清理进度
CREATE TABLE project_data_clean_progress (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    study TEXT NOT NULL,
    site TEXT NOT NULL,
    subject TEXT NOT NULL,
    folder_name TEXT NOT NULL,
    page TEXT NOT NULL,
    entered INTEGER NOT NULL,
    verify_required INTEGER NOT NULL,
    create_time TEXT NOT NULL,
    update_time TEXT,
    report_number TEXT NOT NULL
);

CREATE INDEX idx_report_number_6 ON project_data_clean_progress (report_number);

