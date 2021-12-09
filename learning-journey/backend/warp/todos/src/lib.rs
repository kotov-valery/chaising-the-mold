mod models;
mod filters;
mod handlers;
mod state;

use filters::API;
use tokio::sync::mpsc;

use std::net::IpAddr;
use std::str::FromStr;

const DEFAULT_MESSAGE_CAPACITY: usize = 32;

pub async fn start_web_server(host_addr: &str, port_number: u16) {
    log::debug!("Starting the web server on {} port....", port_number);

    let (tx, rx) = mpsc::channel(DEFAULT_MESSAGE_CAPACITY);

    let mut state = state::State::new(rx);
    let api = API::new(tx.clone());

    let state = tokio::spawn(async move {
        state.run().await;
    });
    warp::serve(api.get_routes()).run((IpAddr::from_str(host_addr).unwrap(), port_number)).await;
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

    #[tokio::test]
    async fn test_get_todos(){
        let (tx, rx) = mpsc::channel(super::DEFAULT_MESSAGE_CAPACITY);

        let mut state = state::State::new(rx);
        let api = API::new(tx.clone());

        let mut list = Vec::new();
        list.push(todo1());
        list.push(todo2());
        for todo in list.iter() {
            state.test_add_todo(todo.clone());
        }

        let _ = tokio::spawn(async move {
            state.run().await;
        });

        let resp = request()
            .method("GET")
            .path("/todos")
            .reply(&api.get_routes())
            .await;

        let body = String::from_utf8(resp.body().to_vec()).expect("Could not parse JSON response");
        let body: Vec<Todo> = serde_json::from_str(&body).unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(body, list);
    }

    #[tokio::test]
    async fn test_get_todos_with_options(){
        let (tx, rx) = mpsc::channel(super::DEFAULT_MESSAGE_CAPACITY);

        let mut state = state::State::new(rx);
        let api = API::new(tx.clone());

        state.test_add_todo(todo1());
        state.test_add_todo(todo2());

        let _ = tokio::spawn(async move {
            state.run().await;
        });

        let resp = request()
            .method("GET")
            .path("/todos?offset=1")
            .reply(&api.get_routes())
            .await;

        let body = String::from_utf8(resp.body().to_vec()).expect("Could not parse JSON response");
        let body: Vec<Todo> = serde_json::from_str(&body).unwrap();

        assert_eq!(resp.status(), StatusCode::OK);

        let mut expected = Vec::new();
        expected.push(todo2());
        assert_eq!(body, expected);
    }

    fn todo1() -> Todo {
        Todo {
            id: 1,
            description: "test 1".into(),
            completed: false,
        }
    }

    fn todo2() -> Todo  {
        Todo {
            id: 2,
            description: "test 2".into(),
            completed: false,
        }
    }
}
