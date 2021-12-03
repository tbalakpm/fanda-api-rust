use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub login_name: String,
    pub email_address: String,
    pub mobile_number: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
    pub role_id: i32,
    pub is_enabled: bool,
}

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub login_name: String,
    pub email_address: String,
    pub mobile_number: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password_hash: String,
    pub password_salt: String,
    pub role_id: i32,
    pub is_enabled: bool,
}

impl User {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, login_name, email_address, mobile_number, 
                first_name, last_name, password_hash, password_salt, role_id, is_enabled
            FROM users
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Option<User>> {
        let rec = sqlx::query_as!(
            User,
            r#"
            SELECT id, login_name, email_address, mobile_number, 
                first_name, last_name, password_hash, password_salt, role_id, is_enabled
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*pool)
        .await?;

        Ok(rec)

        // Ok(rec.map(|rec| User {
        //     id: rec.id,
        //     login_name: rec.login_name,
        //     email_address: rec.email_address,
        //     mobile_number: rec.mobile_number,
        //     first_name: rec.first_name,
        //     last_name: rec.last_name,
        //     password_hash: rec.password_hash,
        //     password_salt: rec.password_salt,
        //     role_id: rec.role_id,
        //     is_enabled: rec.is_enabled,
        // }))
    }

    pub async fn create(user: UserRequest, pool: &PgPool) -> Result<User> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO users (login_name, email_address, mobile_number, 
                first_name, last_name, role_id, password_hash, password_salt, is_enabled)
            VALUES ($1, $2, $3, 
                $4, $5, $6, $7, $8, $9)
            RETURNING id;
            "#,
            user.login_name,
            user.email_address,
            user.mobile_number,
            user.first_name,
            user.last_name,
            user.role_id,
            String::from("123:hash"),
            String::from("123:salt"),
            user.is_enabled
        )
        .fetch_one(pool)
        .await?;

        Ok(User {
            id: rec.id,
            login_name: user.login_name,
            email_address: user.email_address,
            mobile_number: user.mobile_number,
            first_name: user.first_name,
            last_name: user.last_name,
            password_hash: String::from("123:hash"),
            password_salt: String::from("123:salt"),
            role_id: user.role_id,
            is_enabled: user.is_enabled,
        })
    }

    // pub async fn update(id: i32, user: UserRequest, pool: &PgPool) -> Result<Option<User>> {
    //     let result = sqlx::query!(
    //         r#"
    //         UPDATE roles
    //         SET role_name = $1, is_enabled = $2
    //         WHERE id = $3
    //         "#,
    //         role.name,
    //         role.is_enabled,
    //         id
    //     )
    //     .execute(pool)
    //     .await?;

    //     if result.rows_affected() == 0 {
    //         return Ok(None);
    //     }

    //     Ok(Some(Role {
    //         id,
    //         name: role.name,
    //         is_enabled: role.is_enabled,
    //     }))
    // }

    // pub async fn delete(id: i32, pool: &PgPool) -> Result<u64> {
    //     let result = sqlx::query!(
    //         r#"
    //         DELETE FROM roles
    //         WHERE id = $1
    //         "#,
    //         id
    //     )
    //     .execute(pool)
    //     .await?;

    //     Ok(result.rows_affected())
    // }
}
