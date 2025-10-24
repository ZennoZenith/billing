use lib_utils::envs::{get_env, get_env_parse};
use std::{sync::OnceLock, time::Duration};
use tracing::warn;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    // -- Db
    pub DB_URL: String,
    pub DB_MAX_CONNECTIONS: u32,
    pub DB_CONNECTION_TIMEOUT: Duration,
}

impl CoreConfig {
    fn load_from_env() -> lib_utils::envs::Result<CoreConfig> {
        let db_max_connections =
            match get_env_parse::<u32>("SERVICE_DB_MAX_CONNECTIONS") {
                Err(lib_utils::envs::Error::MissingEnv(_)) => {
                    warn!(
                        "{:<12} - SERVICE_DB_MAX_CONNECTIONS, using default",
                        "MISSING-ENV"
                    );
                    Ok(5)
                }
                rest => rest,
            };
        let db_connections_timeout = match get_env_parse::<u64>(
            "SERVICE_DB_CONNECTION_TIMEOUT_MS",
        ) {
            Err(lib_utils::envs::Error::MissingEnv(_)) => {
                warn!(
                    "{:<12} - SERVICE_DB_CONNECTION_TIMEOUT_MS, using default",
                    "MISSING-ENV"
                );
                Ok(500)
            }
            rest => rest,
        }?;

        Ok(CoreConfig {
            DB_URL: get_env("SERVICE_DB_URL")?,
            DB_MAX_CONNECTIONS: db_max_connections?,
            DB_CONNECTION_TIMEOUT: Duration::from_millis(
                db_connections_timeout,
            ),
        })
    }
}
