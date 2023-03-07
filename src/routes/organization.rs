use crate::entities::organization::{NewOrganization, Organization};
use crate::entities::pagination::extract_pagination;
use crate::entities::pagination::Pagination;
use std::collections::HashMap;

use tracing::{event, instrument, Level};
use warp::hyper::StatusCode;

use crate::store::Store;

#[instrument]
pub async fn get_organizations(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "practical_rust_book", Level::INFO, "querying orget_organizations");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    match store
        .get_organizations(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_organization(
    id: i32,
    store: Store,
    organization: Organization,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.update_organization(organization, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_organization(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_organization(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("organization {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_organization(
    store: Store,
    new_organization: NewOrganization,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_organization(new_organization).await {
        Ok(_) => Ok(warp::reply::with_status(
            "organization added",
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
