//! src/configuration.rs

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings, 
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

// Returns <Settings -> DatabaseSettings -> connection_string(), Err>
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Init the configuration reader
    let mut settings = config::Config::default();

    // Add configuration values from a file named `configuration`
    settings.merge(config::File::with_name("configuration"))?;

    // Convert configuration values it read into Settings type
    settings.try_into()
}



impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}