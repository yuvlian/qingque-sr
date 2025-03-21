use sqlx::{Error, FromRow, SqlitePool};

#[derive(FromRow, Debug)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub password_hash: String,
    // pub is_banned: u8,
    pub user_token: String,
    // pub device_id: String,
}

impl User {
    pub async fn create(
        pool: &SqlitePool,
        username: &str,
        password_hash: &str,
        user_token: &str,
    ) -> Result<u32, Error> {
        let result = sqlx::query(
            "INSERT INTO user (username, password_hash, user_token) 
             VALUES (?, ?, ?)",
        )
        .bind(username)
        .bind(password_hash)
        .bind(user_token)
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid() as u32)
    }

    pub async fn exists_by_username(pool: &SqlitePool, username: &str) -> Result<bool, Error> {
        let exists: Option<i64> =
            sqlx::query_scalar("SELECT 1 FROM user WHERE username = ? LIMIT 1")
                .bind(username)
                .fetch_optional(pool)
                .await?;

        Ok(exists.is_some())
    }

    pub async fn get_by_uid(pool: &SqlitePool, user_id: u32) -> Result<Option<Self>, Error> {
        let user = sqlx::query_as::<_, Self>("SELECT * FROM user WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    pub async fn get_by_token(pool: &SqlitePool, token: &str) -> Result<Option<Self>, Error> {
        let user = sqlx::query_as::<_, Self>("SELECT * FROM user WHERE user_token = ?")
            .bind(token)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    // pub async fn get_token_by_device_id(
    //     pool: &SqlitePool,
    //     device_id: &str,
    // ) -> Result<Option<String>, Error> {
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
    ) -> Result<(), Error> {
        sqlx::query("UPDATE user SET user_token = ? WHERE user_id = ?")
            .bind(new_token)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    // pub async fn set_banned(pool: &SqlitePool, user_id: u32, is_banned: bool) -> Result<(), Error> {
    //     sqlx::query("UPDATE user SET is_banned = ? WHERE user_id = ?")
    //         .bind(if is_banned { 1 } else { 0 })
    //         .bind(user_id)
    //         .execute(pool)
    //         .await?;
    //     Ok(())
    // }

    pub async fn delete(pool: &SqlitePool, user_id: u32) -> Result<(), Error> {
        sqlx::query("DELETE FROM user WHERE user_id = ?")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
