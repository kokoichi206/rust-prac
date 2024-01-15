use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

// repository で発送しうるエラーの定義。
#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

// layer の機能のため、Clone, Send, Sync, 'static を継承する。
pub trait TodoRepository: Clone + Send + Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
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

impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        // // 未実装なものは todo!() マクロを使う。
        // todo!();
        let mut store = self.write_store_ref();

        let id = (store.len() + 1) as i32;
        let todo = Todo::new(id, payload.text.clone());

        store.insert(id, todo.clone());
        todo
    }

    fn find(&self, id: i32) -> Option<Todo> {
        let store = self.read_store_ref();
        // 借用した Todo なので所有権を持っていない → clone() で所有権を持つ。
        store.get(&id).map(|todo| todo.clone())
    }

    fn all(&self) -> Vec<Todo> {
        let store = self.read_store_ref();
        Vec::from_iter(store.values().map(|todo| todo.clone()))
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
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

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.write_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn todo_crud_scenario() {
        let text = "todo test text".to_string();
        let id = 1;
        let expected = Todo::new(id, text.clone());

        // create
        let repo = TodoRepositoryForMemory::new();
        let todo = repo.create(CreateTodo { text });
        assert_eq!(expected, todo);

        // find
        let todo = repo.find(id).unwrap();
        assert_eq!(expected, todo);

        // all
        let todos = repo.all();
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
            .unwrap();
        assert_eq!(expected, todo);

        // delete
        let res = repo.delete(id);
        assert!(res.is_ok());
    }
}

#[cfg(test)]
impl CreateTodo {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
