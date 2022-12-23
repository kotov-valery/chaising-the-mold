use crate::state::{Sender, Message, Status};
use crate::models::{Todo, ListOptions};

use rocket::State;
use rocket::http;

use tokio::sync::oneshot;
use rocket::serde::json::Json;

#[get("/todos?<opts>")]
pub async fn list_todos(opts: ListOptions, tx: &State<Sender>) -> String {
    log::debug!("List todos");
    let tx = tx.clone();

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::List{ options: opts, resp: resp_tx }).await;

    if let Ok(Some(res)) = resp_rx.await {
        return serde_json::to_string(&res).unwrap_or("{}".to_string());
    }
    "{}".to_string()
}

#[post("/todos", data = "<create>")]
pub async fn create_todo(create: Json<Todo>, tx: &State<Sender>) -> http::Status {
    log::debug!("Create todo: {:?}", create);
    let tx = tx.clone();
    let create = create.into_inner();

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::Create{ create: create, resp: resp_tx }).await;

    if let Ok(Some(res)) = resp_rx.await {
        match res {
            Status::Created => return http::Status::Created,
            _ => return http::Status::BadRequest
        }
    }
    http::Status::BadRequest
}

#[delete("/todos/<id>")]
pub async fn delete_todo(id: u64, tx: &State<Sender>) -> http::Status {
    log::debug!("Delete a todo with {} id", id);
    let tx = tx.clone();

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::Delete{ id: id, resp: resp_tx }).await;

    if let Ok(Some(res)) = resp_rx.await {
        match res {
            Status::Deleted => return http::Status::NoContent,
            _ => return http::Status::NotFound,
        }
    }
    http::Status::BadRequest
}

#[put("/todos/<id>", data = "<update>")]
pub async fn update_todo(id: u64, update: Json<Todo>, tx: &State<Sender>) -> http::Status {
    log::debug!("Update todo with {} id to {:?}", id, update);
    let tx = tx.clone();
    let update = update.into_inner();

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::Update{ id: id, update: update, resp: resp_tx }).await;

    if let Ok(Some(res)) = resp_rx.await {
        match res {
            Status::Updated => return http::Status::Ok,
            _ => return http::Status::NotFound,
        }
    }
    http::Status::BadRequest
}