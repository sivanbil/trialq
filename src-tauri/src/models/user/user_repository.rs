use crate::models::user::schema::users::dsl::*;
use crate::models::user::user_model::{NewUser, User};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct UserRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>, // 直接使用 SqliteConnection
}

impl UserRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        UserRepository { pool }
    }

    pub fn create_user(&self, new_user: NewUser) -> Result<User, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        users
            .order(id.desc())
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    pub fn read_users(&self) -> Result<Vec<User>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        users.load::<User>(&mut conn).map_err(|e| e.to_string())
    }

    pub fn update_user(&self, user_id: i32, updated_user: NewUser) -> Result<User, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::update(users.find(user_id))
            .set(&updated_user)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        users
            .find(user_id)
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    pub fn delete_user(&self, user_id: i32) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(users.find(user_id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    pub fn find_user_by_email(&self, user_email: &str) -> Result<Option<User>, String> {
        use crate::models::user::schema::users::dsl::{email, users}; // 导入表名和列名

        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        users
            .filter(email.eq(user_email)) // 使用正确的表名和列名
            .first::<User>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }
}
