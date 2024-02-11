# SpeedFlux RS
Monitoring of your internet using speedtest-cli, ping and InfluxDB.

*Written in Rust*

## Docker
```
docker pull ghcr.io/jinderamarak/speedflux-rs:latest
```

## Configuration
Available environment variables:
- `INFLUXDB_URL`, `INFLUXDB_TOKEN`, `INFLUXDB_ORG`, `INFLUXDB_BUCKET`
- `LOG_LEVEL` - `debug`, `info`, `warn`, `error` [default: `info`]
- Ping specific:
  - `PING_CRON` - cron expression for ping service
  - `PING_HOSTS` - comma separated list of hosts to ping
  - `PING_TIMEOUT` - ping timeout in milliseconds [default: `1000`]
  - `PING_BYTES` - ping packet size in bytes [default: `32`]
  - `PING_COUNT` - number of pings to send [default: `5`]
- Speedtest specific:
  - `SPEEDTEST_CRON` - cron expression for speedtest service
  - `SPEEDTEST_SERVER` - speedtest server id [optional]
  - `SPEEDTEST_FIELDS` - comma separated list of fields sent to InfluxDB
  - `SPEEDTEST_TAGS` - comma separated list of tags sent to InfluxDB
  
### Speedtest - Fields and Tags
- `output_type`
- `timestamp`
- `ping_jitter`
- `ping_latency`
- `ping_low`
- `ping_high`
- `download_bandwidth` and `upload_bandwidth`
- `download_bytes` and `upload_bytes`
- `download_elapsed` and `upload_elapsed`
- `download_latency_iqm` and `upload_latency_iqm`
- `download_latency_low` and `upload_latency_low`
- `download_latency_high` and `upload_latency_high`
- `download_latency_jitter` and `upload_latency_jitter`
- `packet_loss`
- `isp`
- `interface_internal_ip`
- `interface_name`
- `interface_mac_addr`
- `interface_is_vpn`
- `interface_external_ip`
- `server_id`
- `server_host`
- `server_port`
- `server_name`
- `server_location`
- `server_country`
- `server_ip`
- `result_id`
- `result_url`
- `result_persisted`

### Additional Notes
Be aware that this will automatically accept the license and GDPR statement of the `speedtest-cli`. Make sure you agree with them before running.
I heavily inspired myself from the work of @breadlysm and @aidengilmartin when rewriting the [SpeedFlux](https://github.com/breadlysm/SpeedFlux) into Rust.
