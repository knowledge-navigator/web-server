use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::entities::organization::{Organization, OrganizationId};

#[derive(Clone)]
pub struct Store {
    pub organizations: Arc<RwLock<HashMap<OrganizationId, Organization>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            organizations: Arc::new(RwLock::new(Self::init())),
        }
    }

    fn init() -> HashMap<OrganizationId, Organization> { // TODO: Impl. real database
        let file = include_str!("../organizations.json");
        serde_json::from_str(file).expect("can't read organizations.json")
    }
}
