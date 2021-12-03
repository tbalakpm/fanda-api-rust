use anyhow::Result;
use serde::{Deserialize, Serialize};
// use sqlx::sqlite::SqliteRow;
// use sqlx::{FromRow, Row, SqlitePool};
// use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize)]
pub struct RoleRequest {
    pub name: String,
    pub is_enabled: bool,
}

#[derive(Serialize, FromRow)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub is_enabled: bool,
}

impl Role {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Role>> {
        let roles = sqlx::query_as!(
            Role,
            r#"
            SELECT id, role_name AS name, is_enabled
            FROM roles
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;
        // .into_iter()
        // .map(|rec| Role {
        //     id: rec.id,
        //     name: rec.role_name,
        //     is_enabled: rec.is_enabled,
        // })
        // .collect();

        Ok(roles)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Option<Role>> {
        let rec = sqlx::query_as!(
            Role,
            r#"
            SELECT id, role_name AS name, is_enabled
            FROM roles
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*pool)
        .await?;

        // Ok(rec.map(|rec| Role {
        //     id: rec.id,
        //     name: rec.role_name,
        //     is_enabled: rec.is_enabled,
        // }))
        Ok(rec)
    }

    pub async fn create(role: RoleRequest, pool: &PgPool) -> Result<Role> {
        // let mut tx = pool.begin().await?;

        let rec = sqlx::query!(
            r#"
            INSERT INTO roles (role_name, is_enabled)
            VALUES ($1, $2)
            RETURNING id;
            "#,
            role.name,
            role.is_enabled
        )
        .fetch_one(pool) // &mut tx
        .await?;

        // let row_id: i32 = sqlx::query("SELECT last_insert_rowid()")
        //     .map(|row: PgRow| row.get(0))
        //     .fetch_one(&mut tx)
        //     .await?;

        // tx.commit().await?;

        Ok(Role {
            id: rec.id,
            name: role.name,
            is_enabled: role.is_enabled,
        })
    }

    pub async fn update(id: i32, role: RoleRequest, pool: &PgPool) -> Result<Option<Role>> {
        let result = sqlx::query!(
            r#"
            UPDATE roles 
            SET role_name = $1, is_enabled = $2
            WHERE id = $3
            "#,
            role.name,
            role.is_enabled,
            id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        Ok(Some(Role {
            id,
            name: role.name,
            is_enabled: role.is_enabled,
        }))
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM roles
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }
}
