use axum::{
    routing::{get, post},
    Router,
    extract::{Path, Json, State},
    response::IntoResponse,
    http::StatusCode,

};


use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use std::sync::{Arc,Mutex};
use std::net::SocketAddr;
use axum::serve;

#[derive(Debug,Serialize,Deserialize,Clone)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

type Db = Arc<Mutex<Vec<Todo>>>;

#[tokio::main]
async fn main() {
    let db:Db = Arc::new(Mutex::new(Vec::new()));
    let app = Router::new()
        .route("/todos", get(get_todos).post(add_todo))
        .route("/todos/:id", get(get_todos_by_id))
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);


   let listener = TcpListener::bind(addr).await.unwrap();
serve(listener, app.into_make_service()).await.unwrap();
    
    
}

async fn get_todos(state: State<Db>) -> impl IntoResponse {
    let db = state.lock().unwrap();
    Json(db.clone())
}

async fn get_todos_by_id(
    Path(id): Path<usize>,
    state: State<Db>,
) -> impl IntoResponse {
    let db = state.lock().unwrap();
    if let Some(todo) = db.iter().find(|t| t.id == id) {
        Json(todo.clone()).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Todo not found").into_response()
    }
}

async fn add_todo(state: State<Db>,Json(payload): Json<Todo>,) -> impl IntoResponse {
    let mut db = state.lock().unwrap();
    db.push(payload.clone());
    Json(payload)
}
