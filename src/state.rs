use async_session::MemoryStore;
use deadpool_diesel::{postgres::Pool, Manager};
use oauth2::basic::BasicClient;

use crate::{routes::oauth::oauth_client, DbConnection};

pub type PoolType = deadpool_diesel::Pool<Manager<DbConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: deadpool_diesel::Pool<Manager<DbConnection>>,
    pub oauth_client: BasicClient,
    pub store: MemoryStore,
}

impl axum::extract::FromRef<AppState> for PoolType {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl axum::extract::FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}

impl axum::extract::FromRef<AppState> for MemoryStore {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}

impl AppState {
    pub fn new(url: &str) -> Self {
        let manager = Manager::new(url, deadpool::Runtime::Tokio1);
        let pool = Pool::builder(manager).max_size(4).build().unwrap();

        Self {
            pool,
            oauth_client: oauth_client().unwrap(),
            store: MemoryStore::new(),
        }
    }
}
