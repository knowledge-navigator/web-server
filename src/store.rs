use crate::entities::organization::{NewOrganization, Organization, OrganizationId};
use handle_errors::Error;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_organizations(
        &self,
        limit: Option<u32>,
        offset: u32,
    ) -> Result<Vec<Organization>, Error> {
        match sqlx::query("SELECT * from organizations LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Organization {
                id: OrganizationId(row.get("id")),
                name: row.get("name"),
                description: row.get("description"),
                moderators: row.get("moderators"),
                members: row.get("members"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(organizations) => Ok(organizations),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }
    pub async fn add_organization(
        &self,
        new_organization: NewOrganization,
    ) -> Result<Organization, Error> {
        match sqlx::query(
            "INSERT INTO organizations (name, description, moderators, members)
                 VALUES ($1, $2, $3, $4)
                 RETURNING id, name, description, moderators, members",
        )
        .bind(new_organization.name)
        .bind(new_organization.description)
        .bind(new_organization.moderators)
        .bind(new_organization.members)
        .map(|row: PgRow| Organization {
            id: OrganizationId(row.get("id")),
            name: row.get("name"),
            description: row.get("description"),
            moderators: row.get("moderators"),
            members: row.get("members"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(organizations) => Ok(organizations),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn update_organization(
        &self,
        organization: Organization,
        organization_id: i32,
    ) -> Result<Organization, Error> {
        match sqlx::query(
            "UPDATE organizations SET name = $1, description = $2, moderators = $3, members = $4
        WHERE id = $5
        RETURNING id, name, description, moderators, members",
        )
        .bind(organization.name)
        .bind(organization.description)
        .bind(organization.moderators)
        .bind(organization.members)
        .bind(organization_id)
        .map(|row: PgRow| Organization {
            id: OrganizationId(row.get("id")),
            name: row.get("name"),
            description: row.get("description"),
            moderators: row.get("moderators"),
            members: row.get("members"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(organizations) => Ok(organizations),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn delete_organization(&self, organization_id: i32) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM organizations WHERE id = $1")
            .bind(organization_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }
}
