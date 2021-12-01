use crate::models::Storage;
use crate::handlers;
use warp::Filter;
use std::convert::Infallible;

static ROOT_PATH: &str = "todos";

pub struct API {
    storage: Storage,
}

impl API {
    pub fn new(storage: &Storage) -> Self {
        API { storage: storage.clone() }
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
            .and(API::with_storage(self.storage.clone()))
            .and_then(handlers::list_todos)
    }

    fn add_create_todo_route<'a>(&'a self)
        -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path(ROOT_PATH)
            .and(warp::post())
            .and(warp::body::json())
            .and(API::with_storage(self.storage.clone()))
            .and_then(handlers::create_todo)
    }

    fn add_update_todo_route<'a>(&'a self)
        -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path(ROOT_PATH)
            .and(warp::path::param::<u64>())
            .and(warp::put())
            .and(warp::body::json())
            .and(API::with_storage(self.storage.clone()))
            .and_then(handlers::update_todo)
    }

    fn add_delete_todo_route<'a>(&'a self)
        -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path(ROOT_PATH)
            .and(warp::path::param::<u64>())
            .and(warp::delete())
            .and(API::with_storage(self.storage.clone()))
            .and_then(handlers::delete_todo)
    }

    fn with_storage(storage: Storage)
        -> impl Filter<Extract = (Storage,), Error = Infallible> + Clone
    {
        warp::any().map(move || storage.clone())
    }

}
