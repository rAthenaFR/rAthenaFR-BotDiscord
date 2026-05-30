use crate::config::ServiceEndpointConfig;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct RAthenaFrServiceStatus {
    pub name: String,
    pub online: bool,
}

pub async fn check_services(endpoints: &[&ServiceEndpointConfig]) -> Vec<RAthenaFrServiceStatus> {
    let mut statuses = Vec::with_capacity(endpoints.len());

    for endpoint in endpoints {
        statuses.push(check_service(endpoint).await);
    }

    statuses
}

async fn check_service(endpoint: &ServiceEndpointConfig) -> RAthenaFrServiceStatus {
    let address = format!("{}:{}", endpoint.host, endpoint.port);
    let online = matches!(
        timeout(Duration::from_secs(2), TcpStream::connect(&address)).await,
        Ok(Ok(_))
    );

    RAthenaFrServiceStatus {
        name: endpoint.name.to_string(),
        online,
    }
}
