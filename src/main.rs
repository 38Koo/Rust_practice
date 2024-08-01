use serde::{Deserialize, Serialize};
use tower::util::Optional;
use std::sync::{Arc, RwLock};
use std::{collections::HashMap, env};
use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum RepositoryError {
  #[error("Not Found, id is {0}")]
  NotFound(i32)
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
  fn create(&self, payload: CreateTodo) -> Todo;
  fn find(&self, id: i32) -> Option<Todo>;
  fn all(&self) -> Vec<Todo>;
  fn update(&self, id:i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
  fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Todo {
  id: i32,
  text: String,
  completed: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CreateTodo {
  text: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct UpdateTodo {
  text: Option<String>,
  completed: Option<bool>
}

impl Todo {
  pub fn new(id: i32, text: String) -> Self {
    Self {
      id,
      text,
      completed: false
    }
  } 
}

type TodoData = HashMap<i32, Todo>;

#[derive(Clone, Debug)]
pub struct TodoRepositoryForMemory {
  store: Arc<RwLock<TodoData>>
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
      TodoRepositoryForMemory {
        store: Arc::default(),
      }
    }
}

impl TodoRepository for TodoRepositoryForMemory {
  fn create(&self, payload: CreateTodo) -> Todo {
    todo!();
  }

  fn find(&self, id: i32) -> Option<Todo> {
    todo!();
  }

  fn all(&self) -> Vec<Todo> {
    todo!();
  }

  fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
    todo!();
  }

  fn delete(&self, id:i32) -> anyhow::Result<()> {
    todo!();
  }
}

#[tokio::main]
async fn main() {
    // loggingの初期化
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();


    let app = create_app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app() -> Router {
  Router::new()
    .route("/", get(root))
    .route("/users", post(create_user))
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 100,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct CreateUser {
    username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    username: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
      let req = Request::builder().uri("/").body(Body::empty()).unwrap();
      let res = create_app().oneshot(req).await.unwrap();
      let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
      let body = String::from_utf8(bytes.to_vec()).unwrap();

      assert_eq!(body, "Hello, world!");
    }

    #[tokio::test]
    async fn should_return_user_data() {
      let req = Request::builder()
        .uri("/users")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(r#"{ "username": "テスト 太郎" }"#))
        .unwrap();
      let res = create_app().oneshot(req).await.unwrap();
      let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
      let body = String::from_utf8(bytes.to_vec()).unwrap();
      let user: User = serde_json::from_str(&body).expect("cannot convert User instance.");

      assert_eq!(user,
        User {
          id: 100,
          username: "テスト 太郎".to_string(),
        }
      );
    }
}
