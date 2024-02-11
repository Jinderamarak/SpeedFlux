use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct PartialSpeedtestConfig {
    #[arg(
        name = "SPEEDTEST_CRON",
        long = "speedtest-cron",
        env = "SPEEDTEST_CRON"
    )]
    pub cron: Option<String>,
    #[arg(
        name = "SPEEDTEST_SERVER",
        long = "speedtest-server",
        env = "SPEEDTEST_SERVER"
    )]
    pub server: Option<u64>,
    #[arg(
        name = "SPEEDTEST_FIELDS",
        long = "speedtest-fields",
        env = "SPEEDTEST_FIELDS"
    )]
    pub fields: Option<String>,
    #[arg(
        name = "SPEEDTEST_TAGS",
        long = "speedtest-tags",
        env = "SPEEDTEST_TAGS"
    )]
    pub tags: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SpeedtestConfig {
    pub cron: String,
    pub server: Option<u64>,
    pub fields: Vec<String>,
    pub tags: Vec<String>,
}

impl TryFrom<PartialSpeedtestConfig> for Option<SpeedtestConfig> {
    type Error = String;
    fn try_from(config: PartialSpeedtestConfig) -> Result<Self, Self::Error> {
        if config.cron.is_none() && config.fields.is_none() && config.tags.is_none() {
            if config.server.is_some() {
                return Err("SPEEDTEST_SERVER requires other \"SPEEDTEST_\" parameters".to_string());
            }
            return Ok(None);
        }

        let cron = config
            .cron
            .ok_or("SPEEDTEST_CRON is required for \"SPEEDTEST_\" parameters")?;
        let server = config.server;
        let fields = config
            .fields
            .ok_or("SPEEDTEST_FIELDS is required for \"SPEEDTEST_\" parameters")
            .map(|f| parse_comma_list(&f))??;
        let tags = config
            .tags
            .ok_or("SPEEDTEST_TAGS is required for \"SPEEDTEST_\" parameters")
            .map(|t| parse_comma_list(&t))??;

        Ok(Some(SpeedtestConfig {
            cron,
            server,
            fields,
            tags,
        }))
    }
}

fn parse_comma_list(text: &str) -> Result<Vec<String>, String> {
    Ok(text.split(',').map(|s| s.to_string()).collect())
}
