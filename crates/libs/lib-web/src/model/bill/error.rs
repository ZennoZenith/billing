use crate::model::store::dbx::{self, UniqueViolation};
use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(thiserror::Error, Debug, Serialize, strum_macros::Display)]
pub enum Error {
    BillNotFound {
        bill_id: String,
    },

    BillNotUnique,

    #[error(transparent)]
    Dbx(dbx::Error),
}

impl From<dbx::Error> for Error {
    fn from(value: dbx::Error) -> Self {
        match value.resolve_unique_violation() {
            Some(UniqueViolation { .. }) => Self::BillNotUnique,
            None => Self::Dbx(value),
        }
    }
}

// region:    --- Error Boilerplate
