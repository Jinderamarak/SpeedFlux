use crate::services::ping::config::{PartialPingConfig, PingConfig};
use crate::services::speedtest::config::{PartialSpeedtestConfig, SpeedtestConfig};
use clap::{Parser, ValueEnum};
use url::Url;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct PartialConfig {
    #[arg(long, env = "INFLUXDB_URL", value_parser = parse_http_url)]
    pub influxdb_url: Url,
    #[arg(long, env = "INFLUXDB_TOKEN")]
    pub influxdb_token: String,
    #[arg(long, env = "INFLUXDB_ORG", default_value = "org")]
    pub influxdb_org: String,
    #[arg(long, env = "INFLUXDB_BUCKET", default_value = "speedtest")]
    pub influxdb_bucket: String,
    #[arg(value_enum, long, env = "LOG_LEVEL", default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,
    #[command(flatten)]
    pub speedtest: PartialSpeedtestConfig,
    #[command(flatten)]
    pub ping: PartialPingConfig,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub influxdb_url: Url,
    pub influxdb_token: String,
    pub influxdb_org: String,
    pub influxdb_bucket: String,
    pub log_level: LogLevel,
    pub speedtest: Option<SpeedtestConfig>,
    pub ping: Option<PingConfig>,
}

impl TryFrom<PartialConfig> for Config {
    type Error = String;
    fn try_from(config: PartialConfig) -> Result<Self, Self::Error> {
        let speedtest = config.speedtest.try_into()?;
        let ping = config.ping.try_into()?;

        Ok(Config {
            influxdb_url: config.influxdb_url,
            influxdb_token: config.influxdb_token,
            influxdb_org: config.influxdb_org,
            influxdb_bucket: config.influxdb_bucket,
            log_level: config.log_level,
            speedtest,
            ping,
        })
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl From<LogLevel> for log::Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Error => log::Level::Error,
            LogLevel::Warn => log::Level::Warn,
            LogLevel::Info => log::Level::Info,
            LogLevel::Debug => log::Level::Debug,
        }
    }
}

fn parse_http_url(text: &str) -> Result<Url, String> {
    let url = Url::parse(text).map_err(|e| e.to_string())?;

    match url.scheme() {
        "http" | "https" => Ok(url),
        _ => Err("URL scheme must be http or https".to_string()),
    }
}
