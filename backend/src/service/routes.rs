use crate::service::state::{Message, Sender};

use actix_web::{web, HttpRequest, HttpResponse};
use tokio::sync::oneshot;

pub async fn list_measurements(_: HttpRequest, data: web::Data<Sender>) -> HttpResponse {
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

#[cfg(test)]
mod test {
    use actix_web::{body, http, test, web};
    use tokio::sync::mpsc;

    use super::list_measurements;
    use crate::service::{models::Measurement, state::Message};

    const DEFAULT_MESSAGE_CAPACITY: usize = 10;

    #[actix_web::test]
    async fn test_get_no_measurements() {
        let (tx, mut rx) = mpsc::channel(DEFAULT_MESSAGE_CAPACITY);

        let message_handler = tokio::spawn(async move {
            if let Some(message) = rx.recv().await {
                match message {
                    Message::Get { resp } => {
                        let _ = resp.send(None);
                    }
                    _ => panic!("Got wrong message type"),
                }
            }
        });

        let app_data = web::Data::new(tx.clone());

        let req = test::TestRequest::get()
            .insert_header(http::header::ContentType::plaintext())
            .to_http_request();

        let resp = list_measurements(req, app_data).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let expected_body = String::from("{}");
        let body = body::to_bytes(resp.into_body()).await;
        assert_eq!(web::Bytes::from(expected_body), body.unwrap());

        message_handler.await.unwrap();
    }

    #[actix_web::test]
    async fn test_get_some_measurements() {
        let test_data = vec![Measurement {
            temperature: String::from("20.60"),
            humidity: String::from("45.30"),
        }];

        let (tx, mut rx) = mpsc::channel(DEFAULT_MESSAGE_CAPACITY);

        let message_handler = tokio::spawn(async move {
            if let Some(message) = rx.recv().await {
                match message {
                    Message::Get { resp } => {
                        let _ = resp.send(Some(test_data.clone()));
                    }
                    _ => panic!("Got wrong message type"),
                }
            }
        });

        let app_data = web::Data::new(tx.clone());

        let req = test::TestRequest::get()
            .insert_header(http::header::ContentType::plaintext())
            .to_http_request();

        let resp = list_measurements(req, app_data).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let expected_body = String::from("[{\"temperature\":\"20.60\",\"humidity\":\"45.30\"}]");
        let body = body::to_bytes(resp.into_body()).await;
        assert_eq!(web::Bytes::from(expected_body), body.unwrap());

        message_handler.await.unwrap();
    }
}
