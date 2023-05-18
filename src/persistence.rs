//! Stores the overarching persistence types.

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait, Extension};
use reqwest::StatusCode;
use sqlx::{pool::PoolConnection, PgPool, Postgres};

/// A connection to the database.
pub type Connection = PoolConnection<Postgres>;
/// Custom extractor for getting database connections in handlers.
pub struct ConnectionExtractor(pub Connection);

#[async_trait]
impl<S> FromRequestParts<S> for ConnectionExtractor
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<PgPool>::from_request_parts(parts, state)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get the pool"))?;

        let conn = pool.acquire().await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get a connection",
            )
        })?;

        Ok(Self(conn))
    }
}
