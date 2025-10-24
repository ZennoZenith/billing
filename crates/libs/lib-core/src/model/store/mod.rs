// region:    --- Modules

pub(in crate::model) mod dbx;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use crate::core_config;

// endregion: --- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> sqlx::Result<Db> {
    // * See NOTE 1) below
    let max_connections = if cfg!(test) {
        1
    } else {
        core_config().DB_MAX_CONNECTIONS
    };

    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(core_config().DB_CONNECTION_TIMEOUT)
        .connect(&core_config().DB_URL)
        .await
}

// NOTE 1) This is not an ideal situation; however, with sqlx 0.7.1, when executing `cargo test`, some tests that use sqlx fail at a
//         rather low level (in the tokio scheduler). It appears to be a low-level thread/async issue, as removing/adding
//         tests causes different tests to fail. The cause remains uncertain, but setting max_connections to 1 resolves the issue.
//         The good news is that max_connections still function normally for a regular run.
//         This issue is likely due to the unique requirements unit tests impose on their execution, and therefore,
//         while not ideal, it should serve as an acceptable temporary solution.
//         It's a very challenging issue to investigate and narrow down. The alternative would have been to stick with sqlx 0.6.x, which
//         is potentially less ideal and might lead to confusion as to why we are maintaining the older version in this blueprint.
