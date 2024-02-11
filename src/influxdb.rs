use influxdb2::models::{DataPoint, HealthCheck};
use influxdb2::Client;

pub struct InfluxDB {
    client: Client,
    bucket: String,
}

impl InfluxDB {
    pub fn new(url: &str, org: &str, token: &str, bucket: &str) -> Self {
        let client = Client::new(url, org, token);
        Self {
            client,
            bucket: bucket.to_string(),
        }
    }

    pub async fn check_health(&self) -> anyhow::Result<HealthCheck> {
        Ok(self.client.health().await?)
    }

    pub async fn write(&self, data_point: DataPoint) -> anyhow::Result<()> {
        self.client
            .write(self.bucket.as_str(), tokio_stream::iter(vec![data_point]))
            .await?;
        Ok(())
    }

    pub async fn writes(&self, data_points: Vec<DataPoint>) -> anyhow::Result<()> {
        self.client
            .write(self.bucket.as_str(), tokio_stream::iter(data_points))
            .await?;
        Ok(())
    }
}
