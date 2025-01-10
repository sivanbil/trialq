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


