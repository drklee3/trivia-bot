use serenity::prelude::*;
use serenity::async_trait;

use crate::error::Result;
use crate::keys::*;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct UserState {
    pub user_id: i64,
    pub score: i64,
}

#[async_trait]
pub trait UserStateDb {
    async fn from_id(ctx: &Context, id: u64) -> Result<Option<UserState>>;

    async fn inc(ctx: &Context, id: u64) -> Result<UserState>;
}

#[async_trait]
impl UserStateDb for UserState {
    async fn from_id(ctx: &Context, id: u64) -> Result<Option<UserState>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        from_id_query(pool, id as i64).await
    }

    async fn inc(ctx: &Context, id: u64) -> Result<UserState> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        inc_query(pool, id as i64).await
    }
}

async fn from_id_query(pool: &sqlx::SqlitePool, user_id: i64) -> Result<Option<UserState>> {
    sqlx::query_as!(
        UserState,
        r#"
            SELECT *
              FROM user_states
             WHERE user_id = ?1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn inc_query(pool: &sqlx::SqlitePool, user_id: i64) -> Result<UserState> {
    sqlx::query_as!(
        UserState,
        r#"
            INSERT INTO user_states (user_id, score)
                 VALUES (?1, 1)
            ON CONFLICT (user_id)
              DO UPDATE SET score = score + 1
        "#,
        user_id
    )
    .execute(pool)
    .await?;

    sqlx::query_as!(
        UserState,
        r#"
            SELECT *
              FROM user_states
             WHERE user_id = ?1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}
