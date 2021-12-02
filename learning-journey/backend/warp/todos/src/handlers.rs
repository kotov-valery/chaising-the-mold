use crate::models::Todo;
use crate::state::{Sender, Message, Status};
use std::convert::Infallible;
use warp::http::StatusCode;
use tokio::sync::oneshot;

pub async fn list_todos(tx: Sender) -> Result<impl warp::Reply, Infallible> {
    log::debug!("List todos");

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::List{ resp: resp_tx }).await;

    if let Ok(res) = resp_rx.await {
        return Ok(warp::reply::json(&res))
    }
    let empty: Vec<u8> = Vec::new();
    Ok(warp::reply::json(&empty))
}

pub async fn create_todo(create: Todo, tx: Sender) -> Result<impl warp::Reply, Infallible> {
    log::debug!("Create todo: {:?}", create);

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::Create{ create: create, resp: resp_tx }).await;

    if let Ok(Some(res)) = resp_rx.await {
        match res {
            Status::Created => Ok(StatusCode::CREATED),
            _ => Ok(StatusCode::BAD_REQUEST)
        }
    } else {
        Ok(StatusCode::BAD_REQUEST)
    }
}

pub async fn delete_todo(id: u64, tx: Sender) -> Result<impl warp::Reply, Infallible> {
    log::debug!("Delete a todo with {} id", id);

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::Delete{ id: id, resp: resp_tx }).await;

    if let Ok(Some(res)) = resp_rx.await {
        match res {
            Status::Deleted => Ok(StatusCode::NO_CONTENT),
            _ => Ok(StatusCode::NOT_FOUND),
        }
    } else {
        Ok(StatusCode::BAD_REQUEST)
    }
}

pub async fn update_todo(id: u64, update: Todo, tx: Sender)
    -> Result<impl warp::Reply, Infallible>
{
    log::debug!("Update todo with {} id to {:?}", id, update);

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::Update{ id: id, update: update, resp: resp_tx }).await;

    if let Ok(Some(res)) = resp_rx.await {
        match res {
            Status::Updated => Ok(StatusCode::OK),
            _ => Ok(StatusCode::NOT_FOUND),
        }
    } else {
        Ok(StatusCode::BAD_REQUEST)
    }
}
