use crate::service::state::{Message, Sender};

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use tokio::sync::oneshot;

#[get("/measurements")]
pub async fn list_measurements(_: HttpRequest, data: web::Data<Sender>) -> impl Responder {
    log::debug!("List available data");
    let tx = data.clone();

    let (resp_tx, resp_rx) = oneshot::channel();
    let _ = tx.send(Message::Get { resp: resp_tx }).await;

    if let Ok(Some(res)) = resp_rx.await {
        let body = serde_json::to_string(&res).unwrap_or("{}".to_string());
        return HttpResponse::Ok().body(body);
    }
    HttpResponse::Ok().body("{}".to_string())
}
