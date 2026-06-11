use super::*;
use anyhow::Result;

impl ServicesConfig {
    pub(crate) fn from_env() -> Result<Self> {
        let default_host =
            optional("RATHENAFR_SERVER_HOST").unwrap_or_else(|| "127.0.0.1".to_string());

        Ok(Self {
            login: ServiceEndpointConfig {
                name: "Serveur login",
                host: optional("RATHENAFR_LOGIN_HOST").unwrap_or_else(|| default_host.clone()),
                port: parse_optional("RATHENAFR_LOGIN_PORT")?.unwrap_or(6900),
            },
            char_server: ServiceEndpointConfig {
                name: "Serveur char",
                host: optional("RATHENAFR_CHAR_HOST").unwrap_or_else(|| default_host.clone()),
                port: parse_optional("RATHENAFR_CHAR_PORT")?.unwrap_or(6121),
            },
            map: ServiceEndpointConfig {
                name: "Serveur map",
                host: optional("RATHENAFR_MAP_HOST").unwrap_or(default_host),
                port: parse_optional("RATHENAFR_MAP_PORT")?.unwrap_or(5121),
            },
        })
    }
}
