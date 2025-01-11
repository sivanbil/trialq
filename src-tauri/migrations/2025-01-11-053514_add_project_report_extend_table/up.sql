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
    sort_value INTEGER NOT NULL UNIQUE,
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
create table project_report (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_number TEXT NOT NULL UNIQUE,
    report_number TEXT NOT NULL,
    state INTEGER NOT NULL,
    create_time TEXT NOT NULL,
    update_time TEXT NOT NULL
);

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
    project_number TEXT NOT NULL UNIQUE,
    report_number TEXT NOT NULL,
    source_file_name TEXT NOT NULL,
    source_file_type TEXT NOT NULL,
    create_time TEXT NOT NULL,
    update_time TEXT NOT NULL
);

-- 报告汇总表
create table project_report_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    site TEXT NOT NULL UNIQUE,
    site_name TEXT NOT NULL,
    cra TEXT NOT NULL,
    enrollment TEXT NOT NULL,
    pages TEXT NOT NULL,
    pages_entered TEXT NOT NULL,
    missing_pages TEXT NOT NULL,
    sdv_backlog TEXT NOT NULL,
    edc_status_comment TEXT,
    plan_rmv TEXT,
    latest_rmv TEXT
);

-- 报告汇总表扩展信息
create table project_report_extend_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_report_data_id  INTEGER NOT NULL,
    data_group TEXT NOT NULl,
    percent_pages_entered TEXT NOT NULL,
    percent_pages_sdved TEXT NOT NULL,
    answered_query TEXT NOT NULL,
    opened_query TEXT NOT NULL,
    oq_gt7 INTEGER NOT NULL,
    oq_gt14 INTEGER NOT NULL,
    op_gt21 INTEGER NOT NULL,
    op_gt30 INTEGER NOT NULL
);

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
    update_time TEXT
);

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
    qry_status TEXT NOT NULL
);

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
    update_time TEXT
);


