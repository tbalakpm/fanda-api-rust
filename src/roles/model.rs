use anyhow::Result;
use serde::{Deserialize, Serialize};
// use sqlx::sqlite::SqliteRow;
// use sqlx::{FromRow, Row, SqlitePool};
// use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize)]
pub struct RoleRequest {
    pub name: String,
    pub active: bool,
}

#[derive(Serialize, FromRow)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

impl Role {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Role>> {
        let roles = sqlx::query_as!(
            Role,
            r#"
            SELECT id, role_name AS name, active
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
        //     active: rec.active,
        // })
        // .collect();

        Ok(roles)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Option<Role>> {
        let rec = sqlx::query_as!(
            Role,
            r#"
            SELECT id, role_name AS name, active
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
        //     active: rec.active,
        // }))
        Ok(rec)
    }

    pub async fn create(role: RoleRequest, pool: &PgPool) -> Result<Role> {
        // let mut tx = pool.begin().await?;

        let rec = sqlx::query!(
            r#"
            INSERT INTO roles (role_name, active)
            VALUES ($1, $2)
            RETURNING id;
            "#,
            role.name,
            role.active
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
            active: role.active,
        })
    }
}
