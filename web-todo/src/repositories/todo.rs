use serde::{Deserialize, Serialize};
use validator::Validate;

use axum::async_trait;
use sqlx::{FromRow, PgPool};

use super::RepositoryError;

// layer の機能のため、Clone, Send, Sync, 'static を継承する。
#[async_trait]
pub trait TodoRepository: Clone + Send + Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo>;
    async fn find(&self, id: i32) -> anyhow::Result<Todo>;
    async fn all(&self) -> anyhow::Result<Vec<Todo>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Can not be empty."))]
    #[validate(length(max = 100, message = "Text should be less than 100 characters."))]
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty."))]
    #[validate(length(max = 100, message = "Text should be less than 100 characters."))]
    text: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct TodoRepositoryForDb {
    pool: PgPool,
}

impl TodoRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        TodoRepositoryForDb { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForDb {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
INSERT INTO todos (text, completed)
VALUES ($1, false)
RETURNING *;
            "#,
        )
        .bind(payload.text.clone())
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    async fn find(&self, id: i32) -> anyhow::Result<Todo> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
SELECT * FROM todos WHERE id = $1;
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(todo)
    }

    async fn all(&self) -> anyhow::Result<Vec<Todo>> {
        // ここの型は Todo でいいのか。
        let todos = sqlx::query_as::<_, Todo>(
            r#"
SELECT * FROM todos
ORDER BY id desc;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(todos)
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        let old_todo = self.find(id).await?;
        let todo = sqlx::query_as::<_, Todo>(
            r#"
UPDATE todos SET text = $1, completed = $2
WHERE id = $3
RETURNING *;
            "#,
        )
        .bind(payload.text.unwrap_or(old_todo.text))
        .bind(payload.completed.unwrap_or(old_todo.completed))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
DELETE FROM todos WHERE id = $1;
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use super::test_utils::TodoRepositoryForMemory;

    #[tokio::test]
    async fn todo_crud_scenario() {
        let text = "todo test text".to_string();
        let id = 1;
        let expected = Todo::new(id, text.clone());

        // create
        let repo = TodoRepositoryForMemory::new();
        let todo = repo
            .create(CreateTodo { text })
            .await
            .expect("failed to create todo");
        assert_eq!(expected, todo);

        // find
        let todo = repo.find(id).await.unwrap();
        assert_eq!(expected, todo);

        // all
        let todos = repo.all().await.unwrap();
        assert_eq!(vec![expected.clone()], todos);

        // update
        let new_text = "new todo test text".to_string();
        let new_completed = true;
        let expected = Todo {
            id,
            text: new_text.clone(),
            completed: new_completed,
        };
        let todo = repo
            .update(
                id,
                UpdateTodo {
                    text: Some(new_text),
                    completed: Some(new_completed),
                },
            )
            .await
            .unwrap();
        assert_eq!(expected, todo);

        // delete
        let res = repo.delete(id).await;
        assert!(res.is_ok());
    }
}

#[cfg(test)]
pub mod test_utils {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };

    use anyhow::Context;

    use super::*;

    impl Todo {
        pub fn new(id: i32, text: String) -> Self {
            Self {
                id,
                text,
                completed: false,
            }
        }
    }

    impl CreateTodo {
        pub fn new(text: String) -> Self {
            Self { text }
        }
    }

    type TodoDatas = HashMap<i32, Todo>;

    #[derive(Debug, Clone)]
    pub struct TodoRepositoryForMemory {
        store: Arc<RwLock<TodoDatas>>,
    }

    impl TodoRepositoryForMemory {
        pub fn new() -> Self {
            TodoRepositoryForMemory {
                store: Arc::new(RwLock::new(HashMap::new())),
            }
        }

        // スレッドセーフに Hash Map を取得する。
        fn write_store_ref(&self) -> RwLockWriteGuard<TodoDatas> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<TodoDatas> {
            self.store.read().unwrap()
        }
    }

    #[async_trait]
    impl TodoRepository for TodoRepositoryForMemory {
        async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo> {
            // // 未実装なものは todo!() マクロを使う。
            // todo!();
            let mut store = self.write_store_ref();

            let id = (store.len() + 1) as i32;
            let todo = Todo::new(id, payload.text.clone());

            store.insert(id, todo.clone());
            Ok(todo)
        }

        async fn find(&self, id: i32) -> anyhow::Result<Todo> {
            let store = self.read_store_ref();
            // 借用した Todo なので所有権を持っていない → clone() で所有権を持つ。
            let todo = store
                .get(&id)
                .map(|todo| todo.clone())
                .ok_or(RepositoryError::NotFound(id))?;
            Ok(todo)
        }

        async fn all(&self) -> anyhow::Result<Vec<Todo>> {
            let store = self.read_store_ref();
            Ok(Vec::from_iter(store.values().map(|todo| todo.clone())))
        }

        async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
            let mut store = self.write_store_ref();

            let todo = store.get(&id).context(RepositoryError::NotFound(id))?;
            let text = payload.text.unwrap_or(todo.text.clone());
            let completed = payload.completed.unwrap_or(todo.completed);
            let todo = Todo {
                id,
                text,
                completed,
            };

            store.insert(id, todo.clone());
            Ok(todo)
        }

        async fn delete(&self, id: i32) -> anyhow::Result<()> {
            let mut store = self.write_store_ref();
            store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
            Ok(())
        }
    }
}
