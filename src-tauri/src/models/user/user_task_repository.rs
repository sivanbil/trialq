// repositories/task_repository.rs
use crate::models::task_model::{NewTask, Task};
use crate::schema::tasks::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct TaskRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl TaskRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        TaskRepository { pool }
    }

    // 创建任务
    pub fn create_task(&self, new_task: NewTask) -> Result<Task, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(tasks)
            .values(&new_task)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        tasks
            .order(id.desc())
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据任务编号查询任务
    pub fn find_task_by_task_no(&self, task_no: &str) -> Result<Option<Task>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        tasks
            .filter(task_no.eq(task_no))
            .first::<Task>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 根据任务 ID 查询任务
    pub fn find_task_by_id(&self, task_id: i32) -> Result<Option<Task>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        tasks
            .find(task_id)
            .first::<Task>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 更新任务
    pub fn update_task(&self, task_id: i32, updated_task: NewTask) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::update(tasks.find(task_id))
            .set(&updated_task)
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 删除任务
    pub fn delete_task(&self, task_id: i32) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(tasks.find(task_id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }
}