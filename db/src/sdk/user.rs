use regex::Regex;
use sqlx::{FromRow, SqlitePool};
use std::sync::LazyLock;
use utils::cur_date_full_precision;

static VALID_USERNAME: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[A-Za-z0-9_]+$").unwrap());

#[derive(FromRow, Debug)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub password_hash: String,
    // pub is_banned: u8,
    pub user_token: Option<String>,
    // pub device_id: String,
    pub register_date: String,
}

impl User {
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
    }

    pub fn validate_register_form<'a>(username: &'a str, password: &'a str) -> Result<(), &'a str> {
        // let pass_len = password.len();
        // let usrn_len = username.len();

        // if pass_len < 8 || pass_len > 72 {
        //     return Err("Invalid Password Length. Minimum: 8, Maximum: 72.");
        // }

        // if usrn_len < 4 || usrn_len > 16 {
        //     return Err("Invalid Username Length. Minimum: 4, Maximum: 16.");
        // }

        if !VALID_USERNAME.is_match(username) {
            return Err("Invalid Username Format. Must Be Alphanumeric.");
        }

        Ok(())
    }

    pub async fn verify_password(
        pool: &SqlitePool,
        username: &str,
        password: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let stored_hash: Option<String> =
            sqlx::query_scalar("SELECT password_hash FROM user WHERE username = ?")
                .bind(username)
                .fetch_optional(pool)
                .await?
                .map(|hash| hash);

        match stored_hash {
            Some(hash) => Ok(bcrypt::verify(password, &hash)?),
            None => Ok(false),
        }
    }

    pub fn generate_token() -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        format!("{:X}", rng.random::<u32>())
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

    pub async fn get_by_uid(pool: &SqlitePool, user_id: u32) -> Result<Option<Self>, sqlx::Error> {
        let user = sqlx::query_as::<_, Self>("SELECT * FROM user WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    pub async fn get_by_token(pool: &SqlitePool, token: &str) -> Result<Option<Self>, sqlx::Error> {
        let user = sqlx::query_as::<_, Self>("SELECT * FROM user WHERE user_token = ?")
            .bind(token)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    // pub async fn get_token_by_device_id(
    //     pool: &SqlitePool,
    //     device_id: &str,
    // ) -> Result<Option<String>, sqlx::Error> {
    //     let token = sqlx::query_scalar("SELECT user_token FROM user WHERE device_id = ?")
    //         .bind(device_id)
    //         .fetch_optional(pool)
    //         .await?;
    //     Ok(token)
    // }

    pub async fn update_token(
        pool: &SqlitePool,
        user_id: u32,
        new_token: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user SET user_token = ? WHERE user_id = ?")
            .bind(new_token)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    // pub async fn set_banned(pool: &SqlitePool, user_id: u32, is_banned: bool) -> Result<(), sqlx::Error> {
    //     sqlx::query("UPDATE user SET is_banned = ? WHERE user_id = ?")
    //         .bind(if is_banned { 1 } else { 0 })
    //         .bind(user_id)
    //         .execute(pool)
    //         .await?;
    //     Ok(())
    // }

    pub async fn delete(pool: &SqlitePool, user_id: u32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user WHERE user_id = ?")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
