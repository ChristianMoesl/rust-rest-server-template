use crate::todo::Todo;
use anyhow::Result;
use async_trait::async_trait;
use derive_more::Constructor;
use sqlx::PgPool;
use std::sync::Arc;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TodoRepo: Send + Sync {
    async fn add_todo(&self, text: &str) -> Result<Todo>;
    async fn list_todos(&self) -> Result<Vec<Todo>>;
}

#[derive(Constructor)]
pub struct PostgresTodoRepo {
    pool: Arc<PgPool>,
}

#[async_trait]
impl TodoRepo for PostgresTodoRepo {
    async fn add_todo(&self, text: &str) -> Result<Todo> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
    INSERT INTO todo ( text, completed )
    VALUES ( $1, $2 )
    RETURNING *
            "#,
            text,
            false
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(todo)
    }

    async fn list_todos(&self) -> Result<Vec<Todo>> {
        Ok(sqlx::query_as!(Todo, "SELECT * FROM todo")
            .fetch_all(self.pool.as_ref())
            .await?)
    }
}
