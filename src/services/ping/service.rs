use crate::influxdb::InfluxDB;
use crate::services::ping::config::PingConfig;
use crate::services::ping::model::run_ping;
use crate::services::service::Service;
use async_trait::async_trait;
use influxdb2::models::DataPoint;
use log::debug;
use std::sync::Arc;

pub struct PingService {
    db: Arc<InfluxDB>,
    config: PingConfig,
    name: String,
}

impl PingService {
    pub fn new(db: Arc<InfluxDB>, config: PingConfig, name: &str) -> Self {
        Self {
            db,
            config,
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Service for PingService {
    fn name(&self) -> String {
        format!("ping/{}", self.name)
    }

    async fn execute(&self) -> anyhow::Result<()> {
        #[cfg(target_os = "linux")]
        debug!(target: &self.name, "Expecting Linux ping syntax");
        #[cfg(target_os = "windows")]
        debug!(target: &self.name, "Expecting Windows ping syntax");

        //  Hosts are pinged in sequence to avoid interference
        let mut data_points = Vec::new();
        for host in &self.config.hosts {
            debug!(target: &self.name, "Pinging host: {}", host);
            let ping = run_ping(
                &host,
                self.config.bytes,
                self.config.count,
                self.config.timeout,
            )
            .await?;

            let data_point = DataPoint::builder(&self.name)
                .tag("host", host.to_string())
                .field("packet_loss", ping.packet_loss)
                .field("rtt_min", ping.rtt_min)
                .field("rtt_avg", ping.rtt_avg)
                .field("rtt_max", ping.rtt_max)
                .build()?;
            data_points.push(data_point);
        }

        debug!(target: &self.name, "Writing data to DB");
        self.db.writes(data_points).await?;
        Ok(())
    }
}
