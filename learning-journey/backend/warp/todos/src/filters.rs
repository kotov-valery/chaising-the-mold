use crate::handlers;
use crate::state::Sender;
use warp::Filter;
use std::convert::Infallible;

static ROOT_PATH: &str = "todos";

pub struct API {
    tx: Sender,
}

impl API {
    pub fn new(tx: Sender) -> Self {
        API { tx }
    }

    pub fn get_routes<'a>(&'a self)
        -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        self.add_get_todos_route()
            .or(self.add_create_todo_route())
            .or(self.add_delete_todo_route())
            .or(self.add_update_todo_route())
    }

    fn add_get_todos_route<'a>(&'a self)
        -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path(ROOT_PATH)
            .and(warp::get())
            .and(API::with_sender(self.tx.clone()))
            .and_then(handlers::list_todos)
    }

    fn add_create_todo_route<'a>(&'a self)
        -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path(ROOT_PATH)
            .and(warp::post())
            .and(warp::body::json())
            .and(API::with_sender(self.tx.clone()))
            .and_then(handlers::create_todo)
    }

    fn add_update_todo_route<'a>(&'a self)
        -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path(ROOT_PATH)
            .and(warp::path::param::<u64>())
            .and(warp::put())
            .and(warp::body::json())
            .and(API::with_sender(self.tx.clone()))
            .and_then(handlers::update_todo)
    }

    fn add_delete_todo_route<'a>(&'a self)
        -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path(ROOT_PATH)
            .and(warp::path::param::<u64>())
            .and(warp::delete())
            .and(API::with_sender(self.tx.clone()))
            .and_then(handlers::delete_todo)
    }

    fn with_sender(tx: Sender)
        -> impl Filter<Extract = (Sender,), Error = Infallible> + Clone
    {
        warp::any().map(move || tx.clone())
    }
}
