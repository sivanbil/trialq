use crate::models::projects::project_base::project_site_model::{NewProjectSite, ProjectSite};
use crate::models::projects::project_base::project_site_repository::ProjectSiteRepository;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use serde::Serialize;
use crate::core::excel_process_engine::{FileProcessor, ValidationResult};
use crate::models::projects::Pagination;

#[derive(Serialize)]
pub struct DeleteSiteResponse {
    valid: bool,
    message: String, // 返回的消息
}

#[derive(Serialize)]
pub struct ImportSiteResponse {
    valid: bool,
    message: String, // 返回的消息
}


#[derive(Serialize)]
pub struct SiteListResponse {
    valid: bool,
    pub(crate) sites: Vec<ProjectSite>, // 站点列表
    pub(crate) total: i64,              // 总记录数
    current_page: i64,       // 当前页码
    page_size: i64,          // 每页大小
}

#[derive(Serialize)]
pub struct SaveSiteResponse {
    valid: bool,
    message: String,           // 返回的消息
    site: Option<ProjectSite>, // 保存后的站点信息
}

#[derive(Serialize)]
pub struct GetSiteResponse {
    valid: bool,
    message: String,           // 返回的消息
    site: Option<ProjectSite>, // 查询到的站点信息
}

pub struct SiteService {
    repository: ProjectSiteRepository,
}

impl SiteService {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        let repository = ProjectSiteRepository::new(pool);
        SiteService { repository }
    }

    /// 获取分页站点列表
    pub fn fetch_site_list(
        &self,
        current_page: i64,
        page_size: i64,
        project_no: String,
        keyword: Option<String>,
    ) -> Result<SiteListResponse, String> {
        // 创建分页参数
        let pagination = Pagination {
            current_page,
            page_size,
            keyword: keyword.clone(),
        };

        // 查询分页数据
        let sites = self.repository.read_sites(project_no.clone(), pagination)?;

        // 查询总记录数
        let total_count = self.repository.count_sites(project_no, keyword)?;

        // 返回响应
        Ok(SiteListResponse {
            valid: true,
            sites,
            total: total_count,
            current_page,
            page_size,
        })
    }

    /// 保存站点
    /// 保存站点
    pub fn save_site(
        &self,
        project_number: String,
        site_number: String,
        site_name: Option<String>,
        site_cra: Option<String>,
    ) -> Result<SaveSiteResponse, String> {
        // 检查站点是否存在
        let result = if self.not_exist(site_number.clone())? {
            // 创建 NewProjectSite 对象
            let new_site = NewProjectSite {
                project_name: project_number,
                site_number,
                site_name: site_name.unwrap_or_default(), // 如果为 None，则默认为空字符串
                site_cra: site_cra.unwrap_or_default(),   // 如果为 None，则默认为空字符串
            };

            // 保存站点到数据库
            let site = self.repository.create_site(new_site)?;
            Ok(site)
        } else {
            Err("站点已存在".to_string()) // 返回错误信息
        };

        match result {
            Ok(site) => {
                // 保存成功
                Ok(SaveSiteResponse {
                    valid: true,
                    message: "站点保存成功".to_string(),
                    site: Some(site),
                })
            }
            Err(e) => {
                // 保存失败
                Ok(SaveSiteResponse {
                    valid: false,
                    message: e, // 错误信息
                    site: None,
                })
            }
        }
    }
    /// 删除站点
    pub fn delete_site(&self, site_id: i32) -> Result<DeleteSiteResponse, String> {
        // 调用 repository 的删除方法
        let result = self.repository.delete_site(site_id);

        match result {
            Ok(deleted_count) => {
                if deleted_count > 0 {
                    // 删除成功
                    Ok(DeleteSiteResponse {
                        valid: true,
                        message: "站点删除成功".to_string(),
                    })
                } else {
                    // 未找到站点
                    Ok(DeleteSiteResponse {
                        valid: false,
                        message: "未找到站点".to_string(),
                    })
                }
            }
            Err(e) => {
                // 删除失败
                Ok(DeleteSiteResponse {
                    valid: false,
                    message: e, // 错误信息
                })
            }
        }
    }

    /// 检查站点是否存在，存在则返回 false，不存在则返回 true
    pub fn not_exist(&self, site_no: String) -> Result<bool, String> {
        let existing_site = self.repository.find_site_by_no(&site_no)?; // 处理 Result

        match existing_site {
            Some(_) => Ok(false), // 站点存在
            None => Ok(true),     // 站点不存在
        }
    }

    /// 根据站点 ID 查询站点信息
    pub fn get_site_by_id(&self, site_id: i32) -> Result<GetSiteResponse, String> {
        let result = self.repository.find_site_by_id(site_id);

        match result {
            Ok(Some(site)) => {
                // 查询成功
                Ok(GetSiteResponse {
                    valid: true,
                    message: "站点查询成功".to_string(),
                    site: Some(site),
                })
            }
            Ok(None) => {
                // 未找到站点
                Ok(GetSiteResponse {
                    valid: false,
                    message: "未找到站点".to_string(),
                    site: None,
                })
            }
            Err(e) => {
                // 查询失败
                Ok(GetSiteResponse {
                    valid: false,
                    message: e, // 错误信息
                    site: None,
                })
            }
        }
    }

    // 根据 ID 更新项目站点信息
    pub fn update_site_by_id(&self, site_id: i32, updated_site: NewProjectSite) -> Result<SaveSiteResponse, String> {
        // 调用仓库层的 update_by_id 方法
        let result = self.repository.update_by_id(site_id, updated_site);

        match result {
            // 更新成功，返回更新后的记录
            Ok(site) => {
                Ok(SaveSiteResponse {
                    valid: true, // 操作成功
                    message: "更新成功".to_string(),
                    site: Some(site), // 返回更新后的记录
                })
            }
            // 更新失败，返回错误信息
            Err(e) => {
                Ok(SaveSiteResponse {
                    valid: false, // 操作失败
                    message: format!("更新失败: {}", e), // 返回具体的错误信息
                    site: None,
                })
            }
        }
    }

    pub fn async_process_excel_files(&self, file_path: String, project_name: String) -> Result<ImportSiteResponse, String> {
        // 处理文件内容
        let callback = |results: Vec<ValidationResult>, file_name: &str| {
            for result in results {
                println!("{:?}", result.data);
                let json_value = serde_json::to_value(&result.data).unwrap();

                if let Ok(mut data) = serde_json::from_value::<NewProjectSite>(json_value) {
                    let site = NewProjectSite {
                        project_name: project_name.clone(),
                        site_number: data.site_number,
                        site_name: data.site_name,
                        site_cra: data.site_cra,
                    };
                    match self.repository.create_site(site) {
                        Ok(site) => {
                            println!("site:{:?}", site);
                        }
                        _ => {

                        }
                    }
                }
            }
        };
        FileProcessor::process_file(file_path, callback).map_err(|e| e.to_string()).expect("TODO: panic message");

        Ok(ImportSiteResponse {
            valid: true,
            message: "导入成功".to_string(),
        })
    }
}
