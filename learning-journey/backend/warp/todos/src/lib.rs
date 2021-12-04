mod models;
mod filters;
mod handlers;
mod state;

use filters::API;
use tokio::sync::mpsc;

const LOCAL_HOST: [u8; 4] = [127,0,0,1];
const DEFAULT_MESSAGE_CAPACITY: usize = 32;

pub async fn start_web_server(port_number: u16) {
    log::debug!("Starting the web server on {} port....", port_number);

    let (tx, rx) = mpsc::channel(DEFAULT_MESSAGE_CAPACITY);

    let mut state = state::State::new(rx);
    let api = API::new(tx.clone());

    let state = tokio::spawn(async move {
        state.run().await;
    });
    warp::serve(api.get_routes()).run((LOCAL_HOST, port_number)).await;
    state.await.unwrap();
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;
    use crate::models::Todo;
    use crate::filters::API;
    use crate::state;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_post() {
        let (tx, rx) = mpsc::channel(super::DEFAULT_MESSAGE_CAPACITY);

        let mut state = state::State::new(rx);
        let api = API::new(tx.clone());

        let _ = tokio::spawn(async move {
            state.run().await;
        });

        let resp = request()
            .method("POST")
            .path("/todos")
            .json(&todo1())
            .reply(&api.get_routes())
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_post_conflict() {
        let (tx, rx) = mpsc::channel(super::DEFAULT_MESSAGE_CAPACITY);

        let mut state = state::State::new(rx);
        let api = API::new(tx.clone());

        state.test_add_todo(todo1());
        let _ = tokio::spawn(async move {
            state.run().await;
        });

        let resp = request()
            .method("POST")
            .path("/todos")
            .json(&todo1())
            .reply(&api.get_routes())
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    fn todo1() -> Todo {
        Todo {
            id: 1,
            description: "test 1".into(),
            completed: false,
        }
    }
}