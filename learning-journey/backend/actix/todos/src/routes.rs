use crate::models::{Todo, ListOptions};
use crate::state::{Sender, Message, Status};

use tokio::sync::oneshot;
use actix_web::{
    get, post, put, delete,
    web,
    HttpRequest, HttpResponse, Responder};

#[get("/todos")]
pub async fn list_todos(req: HttpRequest, data: web::Data<Sender>) -> impl Responder {
    log::debug!("List todos");
    let tx = data.clone();
    let opts = web::Query::<ListOptions>::from_query(req.query_string()).unwrap();

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::List{ options: opts.into_inner(), resp: resp_tx }).await;

    if let Ok(Some(res)) = resp_rx.await {
        let body = serde_json::to_string(&res).unwrap_or("{}".to_string());
        return HttpResponse::Ok().body(body);
    }
    HttpResponse::Ok().body("{}".to_string())
}

#[post("/todos")]
pub async fn create_todo(create: web::Json<Todo>, data: web::Data<Sender>) -> impl Responder {
    log::debug!("Create todo: {:?}", create);
    let tx = data.clone();

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::Create{ create: create.into_inner(), resp: resp_tx }).await;

    if let Ok(Some(res)) = resp_rx.await {
        match res {
            Status::Created => return HttpResponse::Created(),
            _ => return HttpResponse::BadRequest(),
        }
    }
    HttpResponse::BadRequest()
}


#[delete("/todos/{id}")]
pub async fn delete_todo(req: HttpRequest, data: web::Data<Sender>) -> impl Responder {
    let id = req.match_info().get("id").unwrap_or("0");
    log::debug!("Delete a todo with {} id", id);
    let tx = data.clone();
    let id = id.parse::<u64>().unwrap();

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::Delete{ id: id, resp: resp_tx }).await;

    if let Ok(Some(res)) = resp_rx.await {
        match res {
            Status::Deleted => return HttpResponse::NoContent(),
            _ => return HttpResponse::NotFound(),
        }
    }
    HttpResponse::BadRequest()
}

#[put("/todos/{id}")]
pub async fn update_todo(req: HttpRequest, update: web::Json<Todo>, data: web::Data<Sender>) -> impl Responder {
    let id = req.match_info().get("id").unwrap_or("0");
    log::debug!("Update todo with {} id to {:?}", id, update);
    let tx = data.clone();
    let id = id.parse::<u64>().unwrap();

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::Update{ id: id, update: update.into_inner(), resp: resp_tx }).await;

    if let Ok(Some(res)) = resp_rx.await {
        match res {
            Status::Updated => return HttpResponse::Ok(),
            _ => return HttpResponse::NotFound(),
        }
    }
    HttpResponse::BadRequest()
}