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

-- 为 project_missing_page 表创建索引
CREATE INDEX idx_project_missing_page_project_name_site_number
    ON project_missing_page (project_name, site_number);

-- 项目query_detail数据
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
                                      qry_response_date TEXT NOT NULL,
                                      qry_response_date_localized TEXT NOT NULL,
                                      qry_response_user TEXT NOT NULL,
                                      qry_answer TEXT NOT NULL,
                                      qry_status TEXT NOT NULL,
                                      create_time TEXT NOT NULL,
                                      update_time TEXT
);

-- 为 project_query_detail 表创建索引
CREATE INDEX idx_project_query_detail_project_name_site_name
    ON project_query_detail (study, site_name);

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

-- 为 project_data_clean_progress 表创建索引
CREATE INDEX idx_project_data_clean_progress_study_site
    ON project_data_clean_progress (study, site);