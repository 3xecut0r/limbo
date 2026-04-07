use thiserror::Error;


#[derive(Debug, Error)]
pub enum DbError {
    #[error("creating pool error: {0}")]
    CreatePoolError(#[from] deadpool_postgres::CreatePoolError),

    #[error("pool error: {0}")]
    Pool(#[from] deadpool_postgres::PoolError),

    #[error("postgres error: {0}")]
    Postgres(#[from] tokio_postgres::Error),

    #[error("not found")]
    NotFound,

}
