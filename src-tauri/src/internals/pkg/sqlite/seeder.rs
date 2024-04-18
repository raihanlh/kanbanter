use std::sync::Arc;

use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::internals::{
    model::{
        board::{Board, GetAllBoardFilter},
        task::{GetTaskFilter, Task},
    },
    repository::sqlite::{
        board::BoardRepositoryImpl,
        repository::{BoardRepository, TaskRepository},
        task::TaskRepositoryImpl,
    },
};

pub async fn seed(db_url: String) {
    let db = SqlitePool::connect(db_url.as_str()).await.unwrap();
    let board_repo = BoardRepositoryImpl::new(db.clone()).await;
    let task_repo = TaskRepositoryImpl::new(db.clone()).await;

    let boards = board_repo
        .get_all(GetAllBoardFilter { is_archived: false })
        .await;
    if boards.len() == 0 {
        let mut board_seeder = BoardSeeder::new(board_repo).await;
        board_seeder.seed().await;

        let mut board_ids: Vec<i64> = vec![];
        for (_idx, board) in board_seeder.generated_boards.iter().enumerate() {
            board_ids.push(board.board_id)
        }

        let mut task_seeder = TaskSeeder::new(task_repo, board_ids).await;
        task_seeder.seed().await;
    }
}

#[async_trait]
trait Seeder<T> {
    async fn seed(&mut self) -> Vec<Box<T>>;
}

// Board Seeder
pub struct BoardSeeder {
    board_repo: Arc<Box<dyn BoardRepository + Send + Sync>>,
    generated_boards: Vec<Box<Board>>,
}

impl BoardSeeder {
    pub async fn new(board_repo: Arc<Box<dyn BoardRepository + Send + Sync>>) -> Self {
        BoardSeeder {
            board_repo,
            generated_boards: vec![],
        }
    }
}

#[async_trait]
impl Seeder<Board> for BoardSeeder {
    async fn seed(&mut self) -> Vec<Box<Board>> {
        let new_boards = vec![
            Board {
                board_id: 0,
                name: "Backlog".to_string(),
                description: "Backlog".to_string(),
                position: 0,
                tasks: vec![],
                created_at: chrono::Local::now(),
                updated_at: chrono::Local::now(),
                deleted_at: Option::None,
            },
            Board {
                board_id: 0,
                name: "To Do".to_string(),
                description: "To Do".to_string(),
                position: 0,
                tasks: vec![],
                created_at: chrono::Local::now(),
                updated_at: chrono::Local::now(),
                deleted_at: Option::None,
            },
            Board {
                board_id: 0,
                name: "In Progress".to_string(),
                description: "To Do".to_string(),
                position: 0,
                tasks: vec![],
                created_at: chrono::Local::now(),
                updated_at: chrono::Local::now(),
                deleted_at: Option::None,
            },
            Board {
                board_id: 0,
                name: "In Review".to_string(),
                description: "To Do".to_string(),
                position: 0,
                tasks: vec![],
                created_at: chrono::Local::now(),
                updated_at: chrono::Local::now(),
                deleted_at: Option::None,
            },
            Board {
                board_id: 0,
                name: "In Testing".to_string(),
                description: "To Do".to_string(),
                position: 0,
                tasks: vec![],
                created_at: chrono::Local::now(),
                updated_at: chrono::Local::now(),
                deleted_at: Option::None,
            },
            Board {
                board_id: 0,
                name: "Done".to_string(),
                description: "To Do".to_string(),
                position: 0,
                tasks: vec![],
                created_at: chrono::Local::now(),
                updated_at: chrono::Local::now(),
                deleted_at: Option::None,
            },
        ];

        let mut board_ids_res = vec![];
        let mut boards_res = vec![];

        for (_idx, board) in new_boards.iter().enumerate() {
            let res = self
                .board_repo
                .insert(Board {
                    board_id: 0,
                    name: board.name.clone(),
                    description: board.description.clone(),
                    position: self.board_repo.get_highest_board_position().await + 1,
                    tasks: board.tasks.clone(),
                    created_at: board.created_at,
                    updated_at: board.updated_at,
                    deleted_at: board.deleted_at,
                })
                .await;

            boards_res.push(res.clone());
            board_ids_res.push(res.board_id.clone());
        }
        self.generated_boards = boards_res.clone();

        boards_res
    }
}

// Task Seeder
pub struct TaskSeeder {
    task_repo: Arc<Box<dyn TaskRepository + Send + Sync>>,
    board_ids: Vec<i64>,
    generated_tasks: Vec<Box<Task>>,
}

impl TaskSeeder {
    pub async fn new(
        task_repo: Arc<Box<dyn TaskRepository + Send + Sync>>,
        board_ids: Vec<i64>,
    ) -> Self {
        TaskSeeder {
            task_repo,
            board_ids,
            generated_tasks: vec![],
        }
    }
}

#[async_trait]
impl Seeder<Task> for TaskSeeder {
    async fn seed(&mut self) -> Vec<Box<Task>> {
        let mut tasks_res = vec![];
        if self.board_ids.len() > 0 {
            for (_idx, board_id) in self.board_ids.iter().enumerate() {
                let new_tasks = vec![
                    Task {
                        task_id: 0,
                        board_id: board_id.clone(),
                        title: format!("Task 1").to_string(),
                        description: "desc test".to_string(),
                        position: 0,
                        created_at: chrono::Local::now(),
                        updated_at: chrono::Local::now(),
                        deleted_at: Option::None,
                    },
                    Task {
                        task_id: 0,
                        board_id: board_id.clone(),
                        title: format!("Task 2").to_string(),
                        description: "desc test 2".to_string(),
                        position: 0,
                        created_at: chrono::Local::now(),
                        updated_at: chrono::Local::now(),
                        deleted_at: Option::None,
                    },
                ];

                for (_idx_task, task) in new_tasks.iter().enumerate() {
                    let res = self
                        .task_repo
                        .insert(Task {
                            task_id: 0,
                            board_id: task.board_id,
                            title: task.title.clone(),
                            description: task.description.clone(),
                            position: self
                                .task_repo
                                .get_highest_task_position(task.board_id)
                                .await
                                + 1,
                            created_at: task.created_at,
                            updated_at: task.updated_at,
                            deleted_at: task.deleted_at,
                        })
                        .await;
                    let _task = self.task_repo.get_by_id(res.task_id).await;
                    tasks_res.push(res);
                }
                break;
            }
            self.generated_tasks = tasks_res.clone();
        }

        tasks_res
    }
}
