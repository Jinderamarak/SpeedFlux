use clap::Args;
use std::time::Duration;
use url::Host;

#[derive(Debug, Clone, Args)]
pub struct PartialPingConfig {
    #[arg(name = "PING_CRON", long = "ping-cron", env = "PING_CRON")]
    pub cron: Option<String>,
    #[arg(name = "PING_HOSTS", long = "ping-hosts", env = "PING_HOSTS")]
    pub hosts: Option<String>,
    #[arg(
        name = "PING_TIMEOUT",
        long = "ping-timeout",
        env = "PING_TIMEOUT",
        default_value = "1000",
        help = "[milliseconds]"
    )]
    pub timeout: u64,
    #[arg(
        name = "PING_BYTES",
        long = "ping-bytes",
        env = "PING_BYTES",
        default_value = "32",
        help = "[bytes]"
    )]
    pub bytes: usize,
    #[arg(
        name = "PING_COUNT",
        long = "ping-count",
        env = "PING_COUNT",
        default_value = "5"
    )]
    pub count: usize,
}

#[derive(Debug, Clone)]
pub struct PingConfig {
    pub cron: String,
    pub hosts: Vec<Host>,
    pub timeout: Duration,
    pub bytes: usize,
    pub count: usize,
}

impl TryFrom<PartialPingConfig> for Option<PingConfig> {
    type Error = String;
    fn try_from(config: PartialPingConfig) -> Result<Self, Self::Error> {
        if config.cron.is_none() && config.hosts.is_none() {
            return Ok(None);
        }

        let cron = config
            .cron
            .ok_or("PING_CRON is required for \"PING_\" parameters")?;
        let hosts = config
            .hosts
            .ok_or("PING_HOSTS is required for \"PING_\" parameters")
            .map(|h| parse_hosts(&h))??;
        let timeout = Duration::from_millis(config.timeout);
        let bytes = config.bytes;
        let count = config.count;

        Ok(Some(PingConfig {
            cron,
            hosts,
            timeout,
            bytes,
            count,
        }))
    }
}

fn parse_hosts(text: &str) -> Result<Vec<Host>, String> {
    text.split(',')
        .map(|s| Host::parse(s))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}
