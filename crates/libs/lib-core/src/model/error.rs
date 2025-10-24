use serde::Serialize;
use serde_with::serde_as;

use crate::model;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(thiserror::Error, Debug, Serialize, strum_macros::Display)]
pub enum Error {
    CantCreateModelManagerProvider(String),

    // -- Modules
    #[error(transparent)]
    User(#[from] model::user::Error),

    #[error(transparent)]
    Seller(#[from] model::seller::Error),

    #[error(transparent)]
    Bill(#[from] model::bill::Error),

    #[error(transparent)]
    Transaction(#[from] model::transaction::Error),
}
