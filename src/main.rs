use crate::config::{Config, PartialConfig};
use crate::influxdb::InfluxDB;
use crate::services::ping::service::PingService;
use crate::services::service::Service;
use crate::services::speedtest::service::SpeedtestService;
use clap::Parser;
use influxdb2::models::Status;
use log::{debug, error, info};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tokio_cron_scheduler::{Job, JobScheduler};

#[cfg(debug_assertions)]
use dotenv::dotenv;

mod config;
mod influxdb;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let config = PartialConfig::parse();
    let config: Config = match config.try_into() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to validate configuration: {}", e);
            return Err(anyhow::anyhow!("{}", e));
        }
    };

    if let Err(e) = simple_logger::init_with_level(config.log_level.into()) {
        eprintln!("Failed to initialize logger:\n{e}");
        return Err(e.into());
    }

    info!("Connecting to InfluxDB: {}", config.influxdb_url);
    let client = InfluxDB::new(
        &config.influxdb_url.to_string(),
        &config.influxdb_org,
        &config.influxdb_token,
        &config.influxdb_bucket,
    );
    check_health(&client).await?;

    let shared_db = Arc::new(client);
    let shared_config = Arc::new(config);

    debug!("Creating job scheduler");
    let mut scheduler = JobScheduler::new().await?;
    if let Some(job) = create_speedtest(shared_config.clone(), shared_db.clone())? {
        scheduler.add(job).await?;
        info!("Added speedtest service");
    }
    if let Some(job) = create_ping(shared_config.clone(), shared_db.clone())? {
        scheduler.add(job).await?;
        info!("Added ping service");
    }

    scheduler.start().await?;

    loop {
        let next = scheduler.time_till_next_job().await?;
        if let Some(next) = &next {
            debug!("Next job in {:?}", next);
        } else {
            debug!("No jobs scheduled");
        }

        let next = next.unwrap_or(Duration::from_secs(60));
        sleep(next).await;
    }
}

fn create_speedtest(config: Arc<Config>, db: Arc<InfluxDB>) -> anyhow::Result<Option<Job>> {
    debug!("Creating speedtest service");
    if let Some(config) = &config.speedtest {
        let service = SpeedtestService::new(db, config.clone(), "speedtest");
        let job = create_service_job(&config.cron, service)?;
        return Ok(Some(job));
    }
    Ok(None)
}

fn create_ping(config: Arc<Config>, db: Arc<InfluxDB>) -> anyhow::Result<Option<Job>> {
    debug!("Creating ping service");
    if let Some(config) = &config.ping {
        let service = PingService::new(db, config.clone(), "ping");
        let job = create_service_job(&config.cron, service)?;
        return Ok(Some(job));
    }
    Ok(None)
}

fn create_service_job<S>(cron: &str, service: S) -> anyhow::Result<Job>
where
    S: Service + Send + Sync + 'static,
{
    debug!(
        "Creating job for service: \"{}\", with cron: {}",
        service.name(),
        cron
    );
    let service = Arc::new(service);
    let job = Job::new_async(cron, move |_, _| {
        let service = service.clone();
        Box::pin(async move {
            info!("Executing service \"{}\"", service.name());
            let result = service.execute().await;
            match result {
                Ok(_) => {
                    info!("Service \"{}\" completed successfully", service.name());
                }
                Err(e) => {
                    error!("Service \"{}\" failed: {}", service.name(), e);
                }
            }
        })
    })?;

    Ok(job)
}

async fn check_health(client: &InfluxDB) -> anyhow::Result<()> {
    match client.check_health().await {
        Err(e) => {
            error!("Failed to connect to InfluxDB: {}", e);
            return Err(e);
        }
        Ok(health) => {
            debug!(
                "InfluxDB health check, name: {}, version: {:?}",
                health.name, health.version
            );
            match health.status {
                Status::Fail => {
                    error!(
                        "InfluxDB health check failed: {}",
                        health.message.unwrap_or("No message".to_string())
                    );
                }
                Status::Pass => {
                    info!(
                        "Connected to InfluxDB: {}",
                        health.message.unwrap_or("No message".to_string())
                    );
                }
            }
        }
    }

    Ok(())
}
