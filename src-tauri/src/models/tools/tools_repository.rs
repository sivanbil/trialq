use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};
use crate::models::tools::tools_model::{Tool, NewTool};
use crate::models::tools::schema::tools::dsl::*;

#[derive(Debug)]
pub struct Pagination {
    pub current_page: i64, // 当前页码
    pub page_size: i64,    // 每页大小
}

pub struct ToolsRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>, // 直接使用 SqliteConnection
}

impl ToolsRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ToolsRepository { pool }
    }

    pub fn create_tool(&self, new_tool: NewTool) -> Result<Tool, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(tools)
            .values(&new_tool)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        tools.order(id.desc()).first(&mut conn).map_err(|e| e.to_string())
    }

    pub fn read_tools(&self, pagination: Pagination) -> Result<Vec<Tool>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 计算 offset
        let offset = (pagination.current_page - 1) * pagination.page_size;

        // 使用 offset 和 limit 进行分页查询
        tools
            .order(id.asc()) // 按 id 升序排序
            .offset(offset)  // 跳过前面的记录
            .limit(pagination.page_size) // 限制每页的记录数
            .load::<Tool>(&mut conn)
            .map_err(|e| e.to_string())
    }

    pub fn count_tools(&self) -> Result<i64, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 查询总记录数
        tools
            .count()
            .get_result(&mut conn)
            .map_err(|e| e.to_string())
    }

    pub fn delete_tool(&self, tool_id: i32) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(tools.find(tool_id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    pub fn find_tool_by_url(&self, tool_url: &str) -> Result<Option<Tool>, String> {
        use crate::models::tools::schema::tools::dsl::{tools, link_url}; // 导入表名和列名

        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        tools.filter(link_url.eq(tool_url)) // 使用正确的表名和列名
            .first::<Tool>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }
}