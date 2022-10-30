use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
// use crate::helpers::crypto;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRequest {
    pub login_name: String,
    pub email_address: String,
    pub mobile_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    pub password: String,
    pub role_id: i32,
    pub is_enabled: bool,
}

#[derive(Serialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub login_name: String,
    pub email_address: String,
    pub mobile_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_salt: Option<String>,
    pub role_id: i32,
    pub is_enabled: bool,
}

impl User {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>> {
        let users = sqlx::query!(
            r#"
            SELECT id, login_name, email_address, mobile_number, 
                first_name, last_name, role_id, is_enabled
            FROM users
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|rec| User {
            id: rec.id,
            login_name: rec.login_name,
            email_address: rec.email_address,
            mobile_number: rec.mobile_number,
            first_name: rec.first_name,
            last_name: rec.last_name,
            password_hash: None,
            password_salt: None,
            role_id: rec.role_id,
            is_enabled: rec.is_enabled,
        })
        .collect();

        Ok(users)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Option<User>> {
        let rec = sqlx::query!(
            r#"
            SELECT id, login_name, email_address, mobile_number, 
                first_name, last_name, role_id, is_enabled
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*pool)
        .await?;

        // Ok(rec)

        Ok(rec.map(|rec| User {
            id: rec.id,
            login_name: rec.login_name,
            email_address: rec.email_address,
            mobile_number: rec.mobile_number,
            first_name: rec.first_name,
            last_name: rec.last_name,
            password_hash: None,
            password_salt: None,
            role_id: rec.role_id,
            is_enabled: rec.is_enabled,
        }))
    }

    pub async fn create(user: UserRequest, pool: &PgPool) -> Result<User> {
        let password_salt = SaltString::generate(&mut OsRng);
        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();
        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2.hash_password(user.password.as_bytes(), &password_salt);
        let password_hash = match password_hash {
            Ok(hash) => hash.to_string(),
            Err(_) => String::default(),
        };

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
            password_hash,
            String::from(password_salt.as_str()),
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
            password_hash: None,
            password_salt: None,
            role_id: user.role_id,
            is_enabled: user.is_enabled,
        })
    }

    // Update can change fistname and lastname
    pub async fn update(id: i32, user: UserRequest, pool: &PgPool) -> Result<Option<User>> {
        let rec = sqlx::query!(
            r#"
            UPDATE users
            SET first_name = $1, last_name = $2, is_enabled = $3
            WHERE id = $4
            RETURNING login_name, email_address, mobile_number, first_name, last_name, role_id, is_enabled
            "#,
            user.first_name,
            user.last_name,
            user.is_enabled,
            id
        )
        // .execute(pool)
        .fetch_optional(&*pool)
        .await?;

        // if result.rows_affected() == 0 {
        //     return Ok(None);
        // }

        Ok(rec.map(|user| User {
            id,
            login_name: user.login_name,
            email_address: user.email_address,
            mobile_number: user.mobile_number,
            first_name: user.first_name,
            last_name: user.last_name,
            password_hash: None,
            password_salt: None,
            role_id: user.role_id,
            is_enabled: user.is_enabled,
        }))
    }

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
