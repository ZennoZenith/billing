use lib_utils::envs::{get_env, get_env_parse};
use std::{net::SocketAddr, sync::OnceLock, time::Duration};
use tracing::info;

pub fn web_config() -> &'static WebConfig {
    static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        WebConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
pub struct WebConfig {
    pub HOST_PORT: SocketAddr,
    // -- Db
    pub DB_URL: String,
    pub DB_MAX_CONNECTIONS: u32,
    pub DB_CONNECTION_TIMEOUT: Duration,
}

impl WebConfig {
    fn load_from_env() -> lib_utils::envs::Result<WebConfig> {
        let db_max_connections =
            match get_env_parse::<u32>("SERVICE_DB_MAX_CONNECTIONS") {
                Err(lib_utils::envs::Error::MissingEnv(_)) => {
                    info!(
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
                info!(
                    "{:<12} - SERVICE_DB_CONNECTION_TIMEOUT_MS, using default",
                    "MISSING-ENV"
                );
                Ok(500)
            }
            rest => rest,
        }?;

        Ok(WebConfig {
            HOST_PORT: get_env_parse("SERVICE_HOST_PORT")?,

            DB_URL: get_env("SERVICE_DB_URL")?,
            DB_MAX_CONNECTIONS: db_max_connections?,
            DB_CONNECTION_TIMEOUT: Duration::from_millis(
                db_connections_timeout,
            ),
        })
    }
}
