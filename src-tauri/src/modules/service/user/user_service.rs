use crate::models::user::user_model::{NewUser, User};
use crate::models::user::user_repository::UserRepository;
use date_formatter::utils::format_date;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use regitry_code::{encrypt_password, generate_random_string};
use std::time::SystemTime;

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        let repository = UserRepository::new(pool);
        UserService { repository }
    }

    pub fn create_user(&self, new_user: NewUser) -> Result<User, String> {
        self.repository.create_user(new_user)
    }

    pub fn read_users(&self) -> Result<Vec<User>, String> {
        self.repository.read_users()
    }

    pub fn update_user(&self, user_id: i32, updated_user: NewUser) -> Result<User, String> {
        self.repository.update_user(user_id, updated_user)
    }

    // 检查用户是否存在，存在则更新，不存在则创建
    pub fn upsert_or_save_user(
        &self,
        email: String,
        license_key: String,
        expire_date: String,
    ) -> Result<(), String> {
        let existing_user = self.repository.find_user_by_email(&email)?;

        match existing_user {
            Some(user) => {
                // 用户存在，更新 active_code
                let updated_user = NewUser {
                    email: user.email,
                    password: user.password,
                    random: user.random,
                    active_code: license_key, // 更新 active_code
                    validate_date: user.validate_date,
                    expire_date,
                };

                self.update_user(user.id, updated_user)
                    .map_err(|e| format!("Failed to update user: {}", e))?;
            }
            None => {
                // 用户不存在，创建新用户
                let random_str = generate_random_string(6);
                let now = SystemTime::now();
                let formatted_date = format_date(now, "%Y-%m-%d");
                let new_user = NewUser {
                    email,
                    password: encrypt_password("123456", &*random_str),
                    random: random_str,
                    active_code: license_key,
                    validate_date: formatted_date,
                    expire_date,
                };

                self.create_user(new_user)
                    .map_err(|e| format!("Failed to create user: {}", e))?;
            }
        }

        Ok(())
    }
}
