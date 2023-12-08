use std::env;
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::domain::error::internal_error;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub struct DatabaseConnection(pub PooledConnection<'static, PostgresConnectionManager<NoTls>>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection where ConnectionPool: FromRef<S>, S: Send + Sync {
    type Rejection = StatusCode;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = ConnectionPool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub async fn init_db() -> Result<ConnectionPool, StatusCode> {
    let manager = PostgresConnectionManager::new_from_stringlike(&env::var("DB_URL").unwrap(), NoTls).map_err(internal_error)?;
    let pool = Pool::builder().build(manager).await.map_err(internal_error)?;


    Ok(pool)
}