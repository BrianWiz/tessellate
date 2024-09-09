use bevy::ecs::query::QuerySingleError;
use derive_more::derive::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    #[from]
    QuerySingleError(QuerySingleError),
}