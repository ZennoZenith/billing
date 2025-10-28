use lib_utils::envs::get_env_parse;
use std::{path::PathBuf, sync::OnceLock};

pub fn reload_config() -> &'static ReloadConfig {
    static INSTANCE: OnceLock<ReloadConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        ReloadConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
pub struct ReloadConfig {
    /// Always hard reload the page instead of hot-reload
    pub HARD_RELOAD: bool,

    /// Ignore hidden and ignored files
    pub AUTO_IGNORE: bool,

    /// Create listener using `PollWatcher`
    ///
    /// `PollWatcher` is a fallback that manually checks file paths for changes at a regular interval.
    /// It is useful for cases where real-time OS notifications fail, such as when a symbolic link is
    /// atomically replaced, or when the monitored directory itself is moved or renamed.
    pub POLL: bool,

    /// Dir to watch for hot reloading
    pub HOT_RELOAD_DIR: PathBuf,
}

impl ReloadConfig {
    fn load_from_env() -> lib_utils::envs::Result<ReloadConfig> {
        let hard_reload = match get_env_parse::<bool>(
            "SERVICE_HOT_RELOAD_HARD_RELOAD",
        ) {
            Err(lib_utils::envs::Error::MissingEnv(_)) => {
                tracing::warn!(
                    "{:<12} - SERVICE_HOT_RELOAD_HARD_RELOAD, using default",
                    "MISSING-ENV"
                );
                Ok(false)
            }
            rest => rest,
        };

        let auto_ignore = match get_env_parse::<bool>(
            "SERVICE_HOT_RELOAD_AUTO_IGNORE",
        ) {
            Err(lib_utils::envs::Error::MissingEnv(_)) => {
                tracing::warn!(
                    "{:<12} - SERVICE_HOT_RELOAD_AUTO_IGNORE, using default",
                    "MISSING-ENV"
                );
                Ok(false)
            }
            rest => rest,
        };

        let poll = match get_env_parse::<bool>("SERVICE_HOT_RELOAD_POLL") {
            Err(lib_utils::envs::Error::MissingEnv(_)) => {
                tracing::warn!(
                    "{:<12} - SERVICE_HOT_RELOAD_POLL, using default",
                    "MISSING-ENV"
                );
                Ok(false)
            }
            rest => rest,
        };

        let hot_reload_dir: PathBuf =
            match get_env_parse::<String>("SERVICE_HOT_RELOAD_DIR") {
                Err(lib_utils::envs::Error::MissingEnv(_)) => {
                    tracing::warn!(
                        "{:<12} - SERVICE_HOT_RELOAD_DIR, using default",
                        "MISSING-ENV"
                    );
                    Ok(String::from("frontend/"))
                }
                rest => rest,
            }?
            .into();

        Ok(ReloadConfig {
            HARD_RELOAD: hard_reload?,
            AUTO_IGNORE: auto_ignore?,
            POLL: poll?,
            HOT_RELOAD_DIR: hot_reload_dir,
        })
    }
}
