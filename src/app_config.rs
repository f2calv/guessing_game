//Note: based on https://github.com/mehcode/config-rs/blob/master/examples/hierarchical-env/settings.rs
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Application {
    pub host: String,
    pub some_value: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfig {
    pub application: Application,
    pub database: Database,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        std::env::set_var("APP_LIST", "Hello World");
        //std::env::set_var("APP_APPLICATION_HOST", "4.5.6.7");

        let app_environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "".into());
        log::info!("APP_ENVIRONMENT={}", app_environment);

        let app_config = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("appconfig.toml"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("appconfig_{}.toml", app_environment)).required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("appconfig_local.toml").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            // You may also programmatically change settings
            //.set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        log::info!("debug: {:?}", app_config.get_bool("debug"));
        log::info!("database: {:?}", app_config.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        app_config.try_deserialize()
    }
}
