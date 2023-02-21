#![warn(clippy::all)]

use handle_errors::return_error;
use warp::{http::Method, Filter};

mod entities;
mod routes;
mod store;

#[tokio::main]
async fn main() {
    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_organization = warp::get()
        .and(warp::path("organizations"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::organization::get_organizations);

    let update_organization = warp::put()
        .and(warp::path("organizations"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::organization::update_organization);

    let delete_organization = warp::delete()
        .and(warp::path("organizations"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::organization::delete_organization);

    let add_organization = warp::post()
        .and(warp::path("organizations"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::organization::add_organizations);

    let routes = get_organization
        .or(update_organization)
        .or(add_organization)
        .or(delete_organization)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
