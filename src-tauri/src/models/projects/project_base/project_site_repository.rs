use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::projects::project_base::project_site_model::{ProjectSite, NewProjectSite};
use crate::models::projects::project_base::schema::project_site::dsl::*;
use crate::models::projects::Pagination;
use diesel::debug_query;
use diesel::sqlite::Sqlite;

pub struct ProjectSiteRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>, // 使用连接池
}

impl ProjectSiteRepository {
    // 初始化仓库
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectSiteRepository { pool }
    }

    // 创建项目站点
    pub fn create_site(&self, new_site: NewProjectSite) -> Result<ProjectSite, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(project_site)
            .values(&new_site)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        // 返回最新插入的记录
        project_site
            .order(id.desc())
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 分页查询项目站点（支持 keyword 检索）
    pub fn read_sites(&self, project_no: String,pagination: Pagination) -> Result<Vec<ProjectSite>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 计算 offset
        let offset = (pagination.current_page - 1) * pagination.page_size;

        // 构建查询
        let mut query = project_site.into_boxed();
        query = query.filter(project_name.eq(project_no));
        // 如果提供了 keyword，则添加模糊匹配条件
        if let Some(keyword) = pagination.keyword {
            let search_pattern = format!("%{}%", keyword); // 使用 % 实现模糊匹配
            query = query.filter(site_number.like(search_pattern));
        }

        println!("{}", debug_query::<Sqlite, _>(&query));

        // 执行分页查询
        let result = query
            //.select((id, project_name, site_number, site_name, site_cra)) // 显式指定字段顺序
            .order(id.asc()) // 按 id 升序排序
            .offset(offset) // 跳过前面的记录
            .limit(pagination.page_size) // 限制每页的记录数
            .load::<ProjectSite>(&mut conn)
            .map_err(|e| e.to_string());

        result
    }

    // 查询项目站点总数（支持 keyword 检索）
    pub fn count_sites(&self,project_no: String, keyword: Option<String>) -> Result<i64, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 构建查询
        let mut query = project_site.into_boxed();
        query = query.filter(project_name.eq(project_no));
        // 如果提供了 keyword，则添加模糊匹配条件
        if let Some(keyword) = keyword {
            let search_pattern = format!("{}", keyword); // 使用 % 实现模糊匹配
            query = query.filter(site_number.like(search_pattern));
        }

        // 查询总记录数
        query
            .count()
            .get_result(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 删除项目站点
    pub fn delete_site(&self, site_id: i32) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(project_site.find(site_id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据项目站点名称查找项目站点
    pub fn find_site_by_name(&self, name: &str) -> Result<Option<ProjectSite>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 使用表字段名进行过滤
        project_site
            .filter(site_name.eq(name)) // 使用 site_name 字段
            .first::<ProjectSite>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 根据项目站点 ID 查询项目站点信息
    pub fn find_site_by_id(&self, site_id: i32) -> Result<Option<ProjectSite>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_site
            .find(site_id) // 根据 ID 查找项目站点
            .first::<ProjectSite>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 根据 ID 更新项目站点信息
    pub fn update_by_id(&self, site_id: i32, updated_site: NewProjectSite) -> Result<ProjectSite, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 更新指定 ID 的记录
        diesel::update(project_site.find(site_id))
            .set(&updated_site)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        // 返回更新后的记录
        project_site
            .find(site_id)
            .first::<ProjectSite>(&mut conn)
            .map_err(|e| e.to_string())
    }
}