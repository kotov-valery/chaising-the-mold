use crate::models::{Pagination, Todo};
use crate::state::{Message, Sender, Status};

use axum::extract::{Path, Query, State};
use axum::{http::StatusCode, response::IntoResponse, Json};
use tokio::sync::oneshot;

pub async fn list_todos(
    pagination: Option<Query<Pagination>>,
    State(sender): State<Sender>,
) -> impl IntoResponse {
    let Query(options) = pagination.unwrap_or_default();

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = sender
        .send(Message::List {
            options: options,
            resp: resp_tx,
        })
        .await;
    if let Ok(Some(res)) = resp_rx.await {
        return Json(res);
    }
    Json(Vec::<Todo>::new())
}

pub async fn create_todo(
    State(sender): State<Sender>,
    Json(input): Json<Todo>,
) -> impl IntoResponse {
    let create = Todo {
        id: input.id,
        description: input.description,
        completed: input.completed,
    };
    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = sender
        .send(Message::Create {
            create: create.clone(),
            resp: resp_tx,
        })
        .await;

    if let Ok(Some(res)) = resp_rx.await {
        if res == Status::Created {
            return (
                StatusCode::CREATED,
                format!("Item with id {} was created", create.id),
            );
        };
    }
    (
        StatusCode::BAD_REQUEST,
        format!("Failed to created item with id {}", create.id),
    )
}

pub async fn delete_todo(Path(id): Path<u64>, State(sender): State<Sender>) -> impl IntoResponse {
    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = sender
        .send(Message::Delete {
            id: id,
            resp: resp_tx,
        })
        .await;

    if let Ok(Some(res)) = resp_rx.await {
        if res == Status::Deleted {
            return (
                StatusCode::NO_CONTENT,
                format!("Item with id {} was deleted", id),
            );
        };
    }
    (
        StatusCode::NOT_FOUND,
        format!("Item with id {} was not found", id),
    )
}

pub async fn update_todo(
    Path(id): Path<u64>,
    State(sender): State<Sender>,
    Json(input): Json<Todo>,
) -> Result<impl IntoResponse, StatusCode> {
    let update = Todo {
        id: input.id,
        description: input.description,
        completed: input.completed,
    };
    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = sender
        .send(Message::Update {
            id: id,
            update: update.clone(),
            resp: resp_tx,
        })
        .await;

    if let Ok(Some(res)) = resp_rx.await {
        if res == Status::Updated {
            return Ok(Json(update));
        }
    }
    Err(StatusCode::BAD_REQUEST)
}
