use regex::Regex;
use sqlx::{FromRow, SqlitePool};
use std::sync::LazyLock;
use utils::cur_date_full_precision;

static VALID_USERNAME: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[A-Za-z0-9_]+$").unwrap());

#[derive(FromRow, Debug)]
pub struct User {
    pub user_id: u32, // pkey autoincrement
    pub username: String,
    pub password_hash: String,
    pub is_banned: u8,
    pub user_token: Option<String>,
    pub combo_token: Option<String>,
    pub device_id: Option<String>,
    pub register_date: String,
}

impl User {
    pub fn generate_token() -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        format!("{:X}", rng.random::<u32>())
    }

    pub async fn give_token(pool: &SqlitePool, username: &str) -> Result<String, sqlx::Error> {
        let token = User::generate_token();
        sqlx::query("UPDATE user SET user_token = ? WHERE username = ?")
            .bind(&token)
            .bind(username)
            .execute(pool)
            .await?;
        Ok(token)
    }

    pub async fn verify_token_by_uid(
        pool: &SqlitePool,
        user_id: u32,
        token: &str,
    ) -> Result<bool, sqlx::Error> {
        let stored_token: Option<String> =
            sqlx::query_scalar("SELECT user_token FROM user WHERE user_id = ?")
                .bind(user_id)
                .fetch_optional(pool)
                .await?;

        Ok(stored_token == Some(token.into()))
    }

    pub async fn insert_device_id_by_username(
        pool: &SqlitePool,
        username: &str,
        device_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user SET device_id = ? WHERE username = ? AND device_id IS NULL")
            .bind(device_id)
            .bind(username)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn verify_token_and_give_combo_token(
        pool: &SqlitePool,
        token: &str,
        device_id: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Fetch stored user token
        let stored_token: Option<String> =
            sqlx::query_scalar("SELECT user_token FROM user WHERE device_id = ?")
                .bind(device_id)
                .fetch_optional(pool)
                .await?;

        // Verify if the token matches
        match stored_token {
            Some(existing_token) if existing_token == token => {
                // Generate and update the combo token
                let combo_token = User::generate_token();
                sqlx::query("UPDATE user SET combo_token = ? WHERE device_id = ?")
                    .bind(&combo_token)
                    .bind(device_id)
                    .execute(pool)
                    .await?;

                Ok(combo_token)
            }
            _ => Err("Invalid token.".into()),
        }
    }

    pub async fn verify_combo_token_by_uid(
        pool: &SqlitePool,
        user_id: u32,
        combo_token: &str,
    ) -> Result<bool, sqlx::Error> {
        let stored_combo_token: Option<String> =
            sqlx::query_scalar("SELECT combo_token FROM user WHERE user_id = ?")
                .bind(user_id)
                .fetch_optional(pool)
                .await?;

        Ok(stored_combo_token == Some(combo_token.into()))
    }

    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
    }

    pub fn validate_register_form<'a>(username: &'a str, password: &'a str) -> Result<(), &'a str> {
        if username.len() == 0 {
            // set a custom length if u want
            return Err("Username cannot be empty.");
        } else if password.len() == 0 {
            return Err("Password cannot be empty.");
        }

        if !VALID_USERNAME.is_match(username) {
            return Err("Invalid Username Format. Must Be Alphanumeric.");
        }
        Ok(())
    }

    pub async fn create(
        pool: &SqlitePool,
        username: &str,
        password_hash: &str,
    ) -> Result<u32, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO user (username, password_hash, register_date) 
             VALUES (?, ?, ?)",
        )
        .bind(username)
        .bind(password_hash)
        .bind(cur_date_full_precision())
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid() as u32)
    }

    pub async fn exists_by_username(
        pool: &SqlitePool,
        username: &str,
    ) -> Result<bool, sqlx::Error> {
        let exists: Option<i64> =
            sqlx::query_scalar("SELECT 1 FROM user WHERE username = ? LIMIT 1")
                .bind(username)
                .fetch_optional(pool)
                .await?;

        Ok(exists.is_some())
    }

    pub async fn verify_login_form_and_give_token(
        pool: &SqlitePool,
        username: &str,
        password: &str,
    ) -> Result<(u32, String), Box<dyn std::error::Error + Send + Sync>> {
        // reuse register validator for consistency
        User::validate_register_form(username, password)?;

        if !User::exists_by_username(pool, username).await? {
            return Err("User does not exist".into());
        }

        let Some((stored_password_hash, user_id)): Option<(String, u32)> =
            sqlx::query_as("SELECT password_hash, user_id FROM user WHERE username = ?")
                .bind(username)
                .fetch_optional(pool)
                .await?
        else {
            return Err("User not found".into());
        };

        match stored_password_hash {
            hash if bcrypt::verify(password, &hash)? => {
                let token = User::give_token(pool, username).await?;
                Ok((user_id, token))
            }
            _ => Err("Invalid password.".into()),
        }
    }
}
