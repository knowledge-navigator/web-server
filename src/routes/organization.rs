use crate::entities::organization::{NewOrganization, Organization};
use crate::entities::pagination::extract_pagination;
use crate::entities::pagination::Pagination;
use crate::profanity::check_profanity;
use crate::store::Store;
use std::collections::HashMap;
use tracing::{event, instrument, Level};
use warp::http::StatusCode;

#[instrument]
pub async fn get_organizations(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "knowledge_nav_web_server_api", Level::INFO, "querying orget_organizations");
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

#[instrument]
pub async fn get_organization_by_id(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "knowledge_nav_web_server_api", Level::INFO, "querying get_organization_by_id");

    match store.get_organization_by_id(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_organization(
    id: i32,
    store: Store,
    organization: Organization,
) -> Result<impl warp::Reply, warp::Rejection> {
    let name = check_profanity(organization.name);
    let description = check_profanity(organization.description);

    let (name, description) = tokio::join!(name, description);

    if name.is_err() {
        return Err(warp::reject::custom(name.unwrap_err()));
    }

    if description.is_err() {
        return Err(warp::reject::custom(description.unwrap_err()));
    }

    let organization = Organization {
        id: organization.id,
        name: name.unwrap(),
        description: description.unwrap(),
        utc_created: organization.utc_created,
        utc_last_updated: organization.utc_last_updated,
        moderators: organization.moderators,
        members: organization.members,
    };

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
    let name = match check_profanity(new_organization.name).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let description = match check_profanity(new_organization.description).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let organization = NewOrganization {
        name,
        description,
        moderators: new_organization.moderators,
        members: new_organization.members,
    };

    match store.add_organization(organization).await {
        Ok(_) => Ok(warp::reply::with_status(
            "organization added",
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
