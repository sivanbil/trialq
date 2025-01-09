-- 用户数据 如果用户开启了账户登录，那么需要先email/密码登录 然后才能使用
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

-- 项目missing_page数据
create table project_missing_page (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_name TEXT NOT NULL,
    site_name TEXT NOT NULL,
    site_number TEXT NOT NULL ,
    subject_id TEXT NOT NULL ,
    instance_name TEXT NOT NULL ,
    data_page_name TEXT NOT NULL ,
    days_of_missing_pages INTEGER NOT NULL ,
    create_time TEXT NOT NULL ,
    update_time TEXT ,
    INDEX project_name (project_name, site_number)
);

-- 项目query_detail数据
create table project_query_detail (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    study TEXT NOT NULL,
    site_name TEXT NOT NULL,
    study_environment_site TEXT NOT NULL ,
    subject_name TEXT NOT NULL ,
    folder TEXT NOT NULL ,
    form TEXT NOT NULL ,
    log_id INTEGER NOT NULL ,
    qry_open_date TEXT NOT NULL ,
    qry_open_date_localized TEXT NOT NULL ,
    qry_open_by TEXT NOT NULL ,
    query_text TEXT NOT NULL ,
    marking_group_name TEXT NOT NULL ,
    qry_response_date TEXT NOT NULL ,
    qry_response_date_localized TEXT NOT NULL ,
    qry_response_user TEXT NOT NULL ,
    qry_answer TEXT NOT NULL ,
    qry_status TEXT NOT NULL ,
    create_time TEXT NOT NULL ,
    update_time TEXT ,
    INDEX project_name (project_name, site_name)
);
-- 项目的数据清理进制
create table project_data_clean_progress (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    study TEXT NOT NULL,
    site TEXT NOT NULL,
    subject TEXT NOT NULL ,
    folder_name TEXT NOT NULL ,
    page TEXT NOT NULL ,
    entered INTEGER NOT NULL ,
    verify_required INTEGER NOT NULL ,
    create_time TEXT NOT NULL ,
    update_time TEXT ,
    INDEX project_name (study, site)
);