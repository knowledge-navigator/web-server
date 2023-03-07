use crate::entities::organization::{Organization, OrganizationId};
use crate::entities::pagination::extract_pagination;
use crate::store::Store;
use handle_errors::Error;
use std::collections::HashMap;
use tracing::{info, warn, instrument};
use warp::http::StatusCode;

#[instrument]
pub async fn get_organizations(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Start querying organizations");
    let res: Vec<Organization> = store.organizations.read().await.values().cloned().collect();

    if params.is_empty() {
        info!(pagination = false);
        return Ok(warp::reply::json(&res));
    }

    let pagination = extract_pagination(params)?;
    info!("Pagination set {:?}", &pagination);
    if (pagination.end > res.len()) || (pagination.start > pagination.end) {
        warn!(
            "Invalid pagination. See organization length: {}",
            &res.len()
        );
        Err(warp::reject::custom(Error::InvalidParameters))
    } else {
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    }
}

pub async fn update_organization(
    id: String,
    store: Store,
    organization: Organization,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store
        .organizations
        .write()
        .await
        .get_mut(&OrganizationId(id))
    {
        Some(q) => *q = organization,
        None => return Err(warp::reject::custom(Error::OrganisationNotFound)),
    }

    Ok(warp::reply::with_status(
        "Organization updated",
        StatusCode::OK,
    ))
}

pub async fn delete_organization(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store
        .organizations
        .write()
        .await
        .remove(&OrganizationId(id))
    {
        Some(_) => (),
        None => return Err(warp::reject::custom(Error::OrganisationNotFound)),
    }

    Ok(warp::reply::with_status(
        "Organization deleted",
        StatusCode::OK,
    ))
}

pub async fn add_organizations(
    store: Store,
    organization: Organization,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .organizations
        .write()
        .await
        .insert(organization.clone().id, organization);

    Ok(warp::reply::with_status(
        "Organization added",
        StatusCode::OK,
    ))
}
