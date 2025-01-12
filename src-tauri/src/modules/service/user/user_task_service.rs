// services/task_service.rs
use crate::models::task_model::{NewTask, Task};
use crate::models::repositories::task_repository::TaskRepository;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use std::sync::Arc;

pub struct TaskService {
    repository: Arc<TaskRepository>,
}

impl TaskService {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        let repository = Arc::new(TaskRepository::new(pool));
        TaskService { repository }
    }

    // 创建任务
    pub fn create_task(&self, new_task: NewTask) -> Result<Task, String> {
        self.repository.create_task(new_task)
    }

    // 根据任务编号查询任务
    pub fn find_task_by_task_no(&self, task_no: &str) -> Result<Option<Task>, String> {
        self.repository.find_task_by_task_no(task_no)
    }

    // 根据任务 ID 查询任务
    pub fn find_task_by_id(&self, task_id: i32) -> Result<Option<Task>, String> {
        self.repository.find_task_by_id(task_id)
    }

    // 更新任务
    pub fn update_task(&self, task_id: i32, updated_task: NewTask) -> Result<usize, String> {
        self.repository.update_task(task_id, updated_task)
    }

    // 删除任务
    pub fn delete_task(&self, task_id: i32) -> Result<usize, String> {
        self.repository.delete_task(task_id)
    }
}