use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;
use handle_errors::Error;
use crate::entities::organization::{Organization, OrganizationId};

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
                utc_created: row.get("utc_created"),
                moderators: row.get("moderators"),
                members: row.get("members"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(Organizations) => Ok(Organizations),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }
    // pub async fn add_Organization(
    //     &self,
    //     new_Organization: NewOrganization,
    // ) -> Result<Organization, Error> {
    //     match sqlx::query(
    //         "INSERT INTO Organizations (title, content, tags)
    //              VALUES ($1, $2, $3)
    //              RETURNING id, title, content, tags",
    //     )
    //     .bind(new_Organization.title)
    //     .bind(new_Organization.content)
    //     .bind(new_Organization.tags)
    //     .map(|row: PgRow| Organization {
    //             id: OrganizationId(row.get("id")),
    //             name: row.get("name"),
    //             description: row.get("description"),
    //             utc_created: row.get("utc_created"),
    //             moderators: row.get("moderators"),
    //             members: row.get("members"),
    //     })
    //     .fetch_one(&self.connection)
    //     .await
    //     {
    //         Ok(Organization) => Ok(Organization),
    //         Err(e) => {
    //             tracing::event!(tracing::Level::ERROR, "{:?}", e);
    //             Err(Error::DatabaseQueryError)
    //         }
    //     }
    // }

    // pub async fn update_Organization(
    //     &self,
    //     Organization: Organization,
    //     Organization_id: i32,
    // ) -> Result<Organization, Error> {
    //     match sqlx::query(
    //         "UPDATE Organizations SET title = $1, content = $2, tags = $3
    //     WHERE id = $4
    //     RETURNING id, title, content, tags",
    //     )
    //     .bind(Organization.title)
    //     .bind(Organization.content)
    //     .bind(Organization.tags)
    //     .bind(Organization_id)
    //     .map(|row: PgRow| Organization {
    //         id: OrganizationId(row.get("id")),
    //         title: row.get("title"),
    //         content: row.get("content"),
    //         tags: row.get("tags"),
    //     })
    //     .fetch_one(&self.connection)
    //     .await
    //     {
    //         Ok(Organization) => Ok(Organization),
    //         Err(e) => {
    //             tracing::event!(tracing::Level::ERROR, "{:?}", e);
    //             Err(Error::DatabaseQueryError)
    //         }
    //     }
    // }

    // pub async fn delete_Organization(
    //     &self,
    //     Organization_id: i32,
    // ) -> Result<bool, Error> {
    //     match sqlx::query("DELETE FROM Organizations WHERE id = $1")
    //         .bind(Organization_id)
    //         .execute(&self.connection)
    //         .await
    //     {
    //         Ok(_) => Ok(true),
    //         Err(e) => {
    //             tracing::event!(tracing::Level::ERROR, "{:?}", e);
    //             Err(Error::DatabaseQueryError)
    //         }
    //     }
    // }
}