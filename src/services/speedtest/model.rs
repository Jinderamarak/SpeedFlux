use chrono::NaiveDateTime;
use influxdb2::models::FieldValue;
use serde::Deserialize;
use std::collections::HashMap;

pub trait AsInfluxDbData {
    fn as_fields(&self) -> HashMap<String, FieldValue>;
    fn as_tags(&self) -> HashMap<String, String>;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CliOutput {
    #[serde(rename = "type")]
    output_type: String,
    #[serde(with = "speedtest_format")]
    timestamp: NaiveDateTime,
    ping: CliOutputPing,
    download: CliOutputUpDown,
    upload: CliOutputUpDown,
    packet_loss: f64,
    isp: String,
    interface: CliOutputInterface,
    server: CliOutputServer,
    result: CliOutputResult,
}

impl AsInfluxDbData for CliOutput {
    fn as_fields(&self) -> HashMap<String, FieldValue> {
        let mut fields = HashMap::new();
        fields.insert(
            "output_type".to_string(),
            FieldValue::String(self.output_type.clone()),
        );
        fields.insert(
            "timestamp".to_string(),
            FieldValue::String(self.timestamp.to_string()),
        );
        fields.insert("packet_loss".to_string(), FieldValue::F64(self.packet_loss));
        fields.insert("isp".to_string(), FieldValue::String(self.isp.clone()));
        for (name, value) in self.ping.as_fields() {
            fields.insert(format!("ping_{name}"), value);
        }
        for (name, value) in self.download.as_fields() {
            fields.insert(format!("download_{name}"), value);
        }
        for (name, value) in self.upload.as_fields() {
            fields.insert(format!("upload_{name}"), value);
        }
        for (name, value) in self.interface.as_fields() {
            fields.insert(format!("interface_{name}"), value);
        }
        for (name, value) in self.server.as_fields() {
            fields.insert(format!("server_{name}"), value);
        }
        for (name, value) in self.result.as_fields() {
            fields.insert(format!("result_{name}"), value);
        }
        fields
    }

    fn as_tags(&self) -> HashMap<String, String> {
        let mut tags = HashMap::new();
        tags.insert("packet_loss".to_string(), self.packet_loss.to_string());
        tags.insert("isp".to_string(), self.isp.clone());
        for (name, value) in self.ping.as_tags() {
            tags.insert(format!("ping_{name}"), value);
        }
        for (name, value) in self.download.as_tags() {
            tags.insert(format!("download_{name}"), value);
        }
        for (name, value) in self.upload.as_tags() {
            tags.insert(format!("upload_{name}"), value);
        }
        for (name, value) in self.interface.as_tags() {
            tags.insert(format!("interface_{name}"), value);
        }
        for (name, value) in self.server.as_tags() {
            tags.insert(format!("server_{name}"), value);
        }
        for (name, value) in self.result.as_tags() {
            tags.insert(format!("result_{name}"), value);
        }
        tags
    }
}

#[derive(Debug, Deserialize)]
pub struct CliOutputPing {
    jitter: f64,
    latency: f64,
    low: f64,
    high: f64,
}

impl AsInfluxDbData for CliOutputPing {
    fn as_fields(&self) -> HashMap<String, FieldValue> {
        let mut fields = HashMap::new();
        fields.insert("jitter".to_string(), FieldValue::F64(self.jitter));
        fields.insert("latency".to_string(), FieldValue::F64(self.latency));
        fields.insert("low".to_string(), FieldValue::F64(self.low));
        fields.insert("high".to_string(), FieldValue::F64(self.high));
        fields
    }

    fn as_tags(&self) -> HashMap<String, String> {
        let mut tags = HashMap::new();
        tags.insert("jitter".to_string(), self.jitter.to_string());
        tags.insert("latency".to_string(), self.latency.to_string());
        tags.insert("low".to_string(), self.low.to_string());
        tags.insert("high".to_string(), self.high.to_string());
        tags
    }
}

#[derive(Debug, Deserialize)]
pub struct CliOutputUpDown {
    bandwidth: f64,
    bytes: u64,
    elapsed: u64,
    latency: CliOutputUpDownLatency,
}

impl AsInfluxDbData for CliOutputUpDown {
    fn as_fields(&self) -> HashMap<String, FieldValue> {
        let mut fields = HashMap::new();
        fields.insert("bandwidth".to_string(), FieldValue::F64(self.bandwidth));
        fields.insert("bytes".to_string(), FieldValue::I64(self.bytes as i64));
        fields.insert("elapsed".to_string(), FieldValue::I64(self.elapsed as i64));
        for (name, value) in self.latency.as_fields() {
            fields.insert(format!("latency_{name}"), value);
        }
        fields
    }

    fn as_tags(&self) -> HashMap<String, String> {
        let mut tags = HashMap::new();
        tags.insert("bandwidth".to_string(), self.bandwidth.to_string());
        tags.insert("bytes".to_string(), self.bytes.to_string());
        tags.insert("elapsed".to_string(), self.elapsed.to_string());
        for (name, value) in self.latency.as_tags() {
            tags.insert(format!("latency_{name}"), value);
        }
        tags
    }
}

#[derive(Debug, Deserialize)]
pub struct CliOutputUpDownLatency {
    iqm: f64,
    low: f64,
    high: f64,
    jitter: f64,
}

impl AsInfluxDbData for CliOutputUpDownLatency {
    fn as_fields(&self) -> HashMap<String, FieldValue> {
        let mut fields = HashMap::new();
        fields.insert("iqm".to_string(), FieldValue::F64(self.iqm));
        fields.insert("low".to_string(), FieldValue::F64(self.low));
        fields.insert("high".to_string(), FieldValue::F64(self.high));
        fields.insert("jitter".to_string(), FieldValue::F64(self.jitter));
        fields
    }

    fn as_tags(&self) -> HashMap<String, String> {
        let mut tags = HashMap::new();
        tags.insert("iqm".to_string(), self.iqm.to_string());
        tags.insert("low".to_string(), self.low.to_string());
        tags.insert("high".to_string(), self.high.to_string());
        tags.insert("jitter".to_string(), self.jitter.to_string());
        tags
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CliOutputInterface {
    internal_ip: String,
    name: String,
    mac_addr: String,
    is_vpn: bool,
    external_ip: String,
}

impl AsInfluxDbData for CliOutputInterface {
    fn as_fields(&self) -> HashMap<String, FieldValue> {
        let mut fields = HashMap::new();
        fields.insert(
            "internal_ip".to_string(),
            FieldValue::String(self.internal_ip.clone()),
        );
        fields.insert("name".to_string(), FieldValue::String(self.name.clone()));
        fields.insert(
            "mac_addr".to_string(),
            FieldValue::String(self.mac_addr.clone()),
        );
        fields.insert("is_vpn".to_string(), FieldValue::Bool(self.is_vpn));
        fields.insert(
            "external_ip".to_string(),
            FieldValue::String(self.external_ip.clone()),
        );
        fields
    }

    fn as_tags(&self) -> HashMap<String, String> {
        let mut tags = HashMap::new();
        tags.insert("internal_ip".to_string(), self.internal_ip.clone());
        tags.insert("name".to_string(), self.name.clone());
        tags.insert("mac_addr".to_string(), self.mac_addr.clone());
        tags.insert("is_vpn".to_string(), self.is_vpn.to_string());
        tags.insert("external_ip".to_string(), self.external_ip.clone());
        tags
    }
}

#[derive(Debug, Deserialize)]
pub struct CliOutputServer {
    id: u64,
    host: String,
    port: u16,
    name: String,
    location: String,
    country: String,
    ip: String,
}

impl AsInfluxDbData for CliOutputServer {
    fn as_fields(&self) -> HashMap<String, FieldValue> {
        let mut fields = HashMap::new();
        fields.insert("id".to_string(), FieldValue::I64(self.id as i64));
        fields.insert("host".to_string(), FieldValue::String(self.host.clone()));
        fields.insert("port".to_string(), FieldValue::I64(self.port as i64));
        fields.insert("name".to_string(), FieldValue::String(self.name.clone()));
        fields.insert(
            "location".to_string(),
            FieldValue::String(self.location.clone()),
        );
        fields.insert(
            "country".to_string(),
            FieldValue::String(self.country.clone()),
        );
        fields.insert("ip".to_string(), FieldValue::String(self.ip.clone()));
        fields
    }

    fn as_tags(&self) -> HashMap<String, String> {
        let mut tags = HashMap::new();
        tags.insert("id".to_string(), self.id.to_string());
        tags.insert("host".to_string(), self.host.clone());
        tags.insert("port".to_string(), self.port.to_string());
        tags.insert("name".to_string(), self.name.clone());
        tags.insert("location".to_string(), self.location.clone());
        tags.insert("country".to_string(), self.country.clone());
        tags.insert("ip".to_string(), self.ip.clone());
        tags
    }
}

#[derive(Debug, Deserialize)]
pub struct CliOutputResult {
    id: String,
    url: String,
    persisted: bool,
}

impl AsInfluxDbData for CliOutputResult {
    fn as_fields(&self) -> HashMap<String, FieldValue> {
        let mut fields = HashMap::new();
        fields.insert("id".to_string(), FieldValue::String(self.id.clone()));
        fields.insert("url".to_string(), FieldValue::String(self.url.clone()));
        fields.insert("persisted".to_string(), FieldValue::Bool(self.persisted));
        fields
    }

    fn as_tags(&self) -> HashMap<String, String> {
        let mut tags = HashMap::new();
        tags.insert("id".to_string(), self.id.clone());
        tags.insert("url".to_string(), self.url.clone());
        tags.insert("persisted".to_string(), self.persisted.to_string());
        tags
    }
}

mod speedtest_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%SZ";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}
