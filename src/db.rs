use sqlx::{error::DatabaseError, postgres::PgPoolOptions, Pool, Postgres};
use tracing::error;

use crate::{
    configuration::DBConfig,
    types::SearchType,
    utils::{err::AppError, response::AppResult},
};

pub async fn connect(db_config: &DBConfig) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect_with(db_config.with_db())
        .await
        .expect("Failed to connect to DB")
}

pub trait DbResultExt<T> {
    fn trace_db(self, msg: &str) -> Self;
    fn with_unique_violation(self, provided_err: AppError, context: &str) -> AppResult<T>;
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(&Box<dyn DatabaseError>) -> AppError,
    ) -> AppResult<T>;
}

impl<T> DbResultExt<T> for Result<T, sqlx::Error> {
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(&Box<dyn DatabaseError>) -> AppError,
    ) -> AppResult<T> {
        self.map_err(|e| match &e {
            sqlx::Error::Database(db_err) => {
                if db_err.constraint() == Some(name) {
                    return map_err(&db_err);
                }

                return AppError::DbError(e);
            }
            _ => e.into(),
        })
    }

    fn trace_db(self, msg: &str) -> Self {
        self.map_err(|e| {
            tracing::error!("DB Error ({msg}) {e}");
            e
        })
    }

    fn with_unique_violation(self, provided_err: AppError, context: &str) -> AppResult<T> {
        self.map_err(|db_err| {
            if let sqlx::Error::Database(e) = &db_err {
                if e.is_unique_violation() {
                    error!("{context} ({provided_err}) {db_err}");
                    return provided_err;
                }
            }

            tracing::error!("DB Error ({context}) {db_err}");
            return db_err.into();
        })
    }
}

pub trait Total {
    fn total(&self) -> usize;
}

pub fn into_search_type<T, R>(data: Vec<T>) -> SearchType<R>
where
    T: Into<R> + Total,
{
    let mut total: usize = 0;
    let mut results: Vec<R> = vec![];

    for item in data {
        total = item.total();
        results.push(item.into());
    }

    return SearchType { results, total };
}
