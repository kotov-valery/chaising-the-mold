use crate::models::{Todo, Storage};
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn list_todos(storage: Storage) -> Result<impl warp::Reply, Infallible> {
    log::debug!("List todos");
    let storage = storage.lock().await;
    let todos = storage.clone();
    Ok(warp::reply::json(&todos))
}

pub async fn create_todo(create: Todo, storage: Storage) -> Result<impl warp::Reply, Infallible> {
    log::debug!("Create todo: {:?}", create);

    let mut storage = storage.lock().await;
    // Check if todo with the id exists,
    // return `400 BadRequest` it one does
    for todo in storage.iter() {
        if todo.id == create.id {
            log::debug!("\t\t-> todo with {} id already exists", create.id);
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    // No todo with such id exists, insert a new one
    // and return `201 Created`
    storage.push(create);
    Ok(StatusCode::CREATED)
}

pub async fn delete_todo(id: u64, storage: Storage) -> Result<impl warp::Reply, Infallible> {
    log::debug!("Delete a todo with {} id", id);

    let mut storage = storage.lock().await;

    let count = storage.len();
    // Retain all todo items with a different id,
    // meaning delete todos with the provided id
    storage.retain(|todo| todo.id != id);

    let deleted = storage.len() != count;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        log::debug!("Todo with {} id was not found", id);
        Ok(StatusCode::NOT_FOUND)
    }
}

pub async fn update_todo(id: u64, update: Todo, storage: Storage)
    -> Result<impl warp::Reply, Infallible>
{
    log::debug!("Update todo with {} id to {:?}", id, update);
    let mut storage = storage.lock().await;

    // Look for specified todo and update it
    for todo in storage.iter_mut() {
        if todo.id == id {
            *todo = update;
            return Ok(StatusCode::OK)
        }
    }

    // If we get here, no appropriate todo was found
    log::debug!("Todo with {} id was not found", id);
    Ok(StatusCode::NOT_FOUND)
}
