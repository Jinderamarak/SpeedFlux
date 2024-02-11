use crate::influxdb::InfluxDB;
use crate::services::service::Service;
use crate::services::speedtest::config::SpeedtestConfig;
use crate::services::speedtest::model::{AsInfluxDbData, CliOutput};
use async_trait::async_trait;
use influxdb2::models::data_point::DataPointError;
use influxdb2::models::DataPoint;
use log::{debug, warn};
use std::sync::Arc;
use tokio::process::Command;

pub struct SpeedtestService {
    db: Arc<InfluxDB>,
    config: SpeedtestConfig,
    name: String,
}

impl SpeedtestService {
    pub fn new(db: Arc<InfluxDB>, config: SpeedtestConfig, name: &str) -> Self {
        Self {
            db,
            config,
            name: name.to_string(),
        }
    }

    fn build_data_point(&self, data: &CliOutput) -> Result<DataPoint, DataPointError> {
        let as_fields = data.as_fields();
        let as_tags = data.as_tags();

        let mut builder = DataPoint::builder(&self.name);
        for field in &self.config.fields {
            if let Some(value) = as_fields.get(field) {
                builder = builder.field(field, value.clone());
            } else {
                warn!(target: &self.name, "Unknown field: {}", field);
            }
        }

        for tag in &self.config.tags {
            if let Some(value) = as_tags.get(tag) {
                builder = builder.tag(tag, value.clone());
            } else {
                warn!(target: &self.name, "Unknown tag: {}", tag);
            }
        }

        builder.build()
    }
}

#[async_trait]
impl Service for SpeedtestService {
    fn name(&self) -> String {
        format!("speedtest/{}", self.name)
    }

    async fn execute(&self) -> anyhow::Result<()> {
        debug!(target: &self.name, "Executing command");
        let mut cmd = Command::new("speedtest");
        cmd.arg("--accept-license")
            .arg("--accept-gdpr")
            .arg("--format=json");

        if let Some(server) = &self.config.server {
            debug!(target: &self.name, "Using server: {}", server);
            cmd.arg("--server").arg(server.to_string());
        }

        let output = cmd.output().await?;
        if !output.status.success() {
            if let Ok(stderr) = String::from_utf8(output.stderr) {
                warn!(target: &self.name, "Command failed: {}, stderr: {}", output.status, stderr);
            } else {
                warn!(target: &self.name, "Command failed: {}", output.status);
            }
        }

        let output = String::from_utf8(output.stdout)?;

        debug!(target: &self.name, "Parsing output");
        let data: Result<CliOutput, _> = serde_json::from_str(&output);
        if let Err(err) = &data {
            warn!(target: &self.name, "Failed to parse output: {}, stdout: {}", err, output);
        }

        let data = data?;

        debug!(target: &self.name, "Building data point");
        let data_point = self.build_data_point(&data)?;
        let data_points = vec![data_point];

        debug!(target: &self.name, "Writing data to DB");
        self.db.writes(data_points).await?;

        Ok(())
    }
}
