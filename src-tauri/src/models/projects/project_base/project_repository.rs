// project_repository.rs
use crate::models::projects::project_base::project_model::{NewProject, Project};
use crate::models::projects::project_base::schema::projects::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::projects::Pagination;

pub struct ProjectRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>, // 使用 SqliteConnection
}

impl ProjectRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectRepository { pool }
    }

    // 创建项目
    pub fn create_project(&self, new_project: NewProject) -> Result<Project, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(projects)
            .values(&new_project)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        projects
            .order(id.desc())
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 分页查询项目（支持 keyword 检索）
    pub fn read_projects(&self, pagination: Pagination) -> Result<Vec<Project>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 计算 offset
        let offset = (pagination.current_page - 1) * pagination.page_size;

        // 构建查询
        let mut query = projects.into_boxed();

        // 如果提供了 keyword，则添加模糊匹配条件
        if let Some(keyword) = pagination.keyword {
            let search_pattern = format!("%{}%", keyword); // 使用 % 实现模糊匹配
            query = query.filter(project_name.like(search_pattern));
        }

        // 执行分页查询
        query
            .order(id.asc()) // 按 id 升序排序
            .offset(offset) // 跳过前面的记录
            .limit(pagination.page_size) // 限制每页的记录数
            .load::<Project>(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 查询项目总数（支持 keyword 检索）
    pub fn count_projects(&self, keyword: Option<String>) -> Result<i64, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 构建查询
        let mut query = projects.into_boxed();

        // 如果提供了 keyword，则添加模糊匹配条件
        if let Some(keyword) = keyword {
            let search_pattern = format!("{}%", keyword); // 使用 % 实现模糊匹配
            query = query.filter(project_name.like(search_pattern));
        }

        // 查询总记录数
        query
            .count()
            .get_result(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 删除项目
    pub fn delete_project(&self, project_id: i32) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(projects.find(project_id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据项目名称查找项目
    pub fn find_project_by_name(&self, name: &str) -> Result<Option<Project>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 使用表字段名进行过滤
        projects
            .filter(project_name.eq(name)) // 使用导入的 project_name 字段
            .first::<Project>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 根据项目 ID 查询项目信息
    pub fn find_project_by_id(&self, project_id: i32) -> Result<Option<Project>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        projects
            .find(project_id) // 根据 ID 查找项目
            .first::<Project>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }
}
