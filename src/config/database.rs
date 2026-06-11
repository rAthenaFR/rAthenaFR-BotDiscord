use super::*;
use anyhow::Result;

impl DatabaseConfig {
    pub(crate) fn from_env() -> Result<Self> {
        Ok(Self {
            host: required("RATHENAFR_DB_HOST")?,
            port: parse_required("RATHENAFR_DB_PORT")?,
            name: required("RATHENAFR_DB_NAME")?,
            user: required("RATHENAFR_DB_USER")?,
            password: required("RATHENAFR_DB_PASSWORD")?,
            max_connections: parse_optional("RATHENAFR_DB_MAX_CONNECTIONS")?.unwrap_or(5),
            acquire_timeout_seconds: parse_optional("RATHENAFR_DB_ACQUIRE_TIMEOUT_SECONDS")?
                .unwrap_or(5),
        })
    }

    pub(crate) fn placeholder() -> Self {
        Self {
            host: String::new(),
            port: 3306,
            name: String::new(),
            user: String::new(),
            password: String::new(),
            max_connections: 1,
            acquire_timeout_seconds: 5,
        }
    }

    pub fn connection_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            urlencoding::encode(&self.user),
            urlencoding::encode(&self.password),
            self.host,
            self.port,
            self.name
        )
    }
}
