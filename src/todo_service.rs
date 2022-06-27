use crate::todo::Todo;
use crate::todo_controller::{CreateTodoDto, TodoDto};
use crate::todo_repo::TodoRepo;
use anyhow::Result;
use async_trait::async_trait;
use derive_more::Constructor;
use std::sync::Arc;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TodoService: Send + Sync {
    async fn add_todo(&self, todo: &CreateTodoDto) -> Result<TodoDto>;
    async fn list_todos(&self) -> Result<Vec<TodoDto>>;
}

#[derive(Constructor)]
pub struct TodoServiceImpl {
    todo_repo: Arc<dyn TodoRepo>,
}

impl From<&Todo> for TodoDto {
    fn from(todo: &Todo) -> Self {
        TodoDto {
            id: todo.id.to_string(),
            text: todo.text.clone(),
            completed: todo.completed,
        }
    }
}

impl From<Todo> for TodoDto {
    fn from(todo: Todo) -> Self {
        TodoDto::from(&todo)
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    async fn add_todo(&self, todo: &CreateTodoDto) -> Result<TodoDto> {
        self.todo_repo.add_todo(&todo.text).await.map(TodoDto::from)
    }

    async fn list_todos(&self) -> Result<Vec<TodoDto>> {
        self.todo_repo
            .list_todos()
            .await
            .map(|todos| todos.iter().map(TodoDto::from).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::todo_repo::MockTodoRepo;
    use mockall::predicate::*;

    #[tokio::test]
    async fn should_add_todo() {
        let dto = CreateTodoDto {
            text: "text".to_string(),
        };

        let todo_repo = Arc::new({
            let mut mock = MockTodoRepo::new();
            mock.expect_add_todo()
                .with(always())
                .times(1)
                .returning(|text| {
                    Ok(Todo {
                        id: 0,
                        text: text.to_string(),
                        completed: false,
                    })
                });
            mock
        });

        let service = TodoServiceImpl::new(todo_repo.clone());

        let result = service.add_todo(&dto).await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            TodoDto {
                id: "0".to_string(),
                text: "text".to_string(),
                completed: false
            }
        );
    }
}
