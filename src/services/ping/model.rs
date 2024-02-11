use regex::Regex;
use std::time::Duration;
use tokio::process::Command;
use url::Host;

pub struct PingOutput {
    pub packet_loss: f64,
    pub rtt_min: f64,
    pub rtt_avg: f64,
    pub rtt_max: f64,
}

pub async fn run_ping(
    target: &Host,
    bytes: usize,
    count: usize,
    timeout: Duration,
) -> anyhow::Result<PingOutput> {
    let output = create_command(&target.to_string(), bytes, count, timeout)
        .output()
        .await?;
    let stdout = String::from_utf8(output.stdout)?;
    parse_output(&stdout)
}

#[cfg(target_os = "linux")]
fn create_command(target: &str, bytes: usize, count: usize, timeout: Duration) -> Command {
    let mut cmd = Command::new("ping");
    cmd.arg("-c").arg(count.to_string());
    cmd.arg("-s").arg(bytes.to_string());
    cmd.arg("-w").arg(timeout.as_secs().to_string());
    cmd.arg(target);
    cmd
}

#[cfg(target_os = "linux")]
fn parse_output(output: &str) -> anyhow::Result<PingOutput> {
    let packet_loss_re = Regex::new(
        r"[0-9]+ packets transmitted, [0-9]+ received, ([0-9]+)% packet loss, time [0-9]+ms",
    )?;
    let rtt_re = Regex::new(r"rtt min/avg/max/mdev = ([0-9.]+)/([0-9.]+)/([0-9.]+)/[0-9.]+ ms")?;
    parse_any_output(output, &packet_loss_re, &rtt_re)
}

#[cfg(target_os = "windows")]
fn create_command(target: &str, bytes: usize, count: usize, timeout: Duration) -> Command {
    let mut cmd = Command::new("ping");
    cmd.arg("-n").arg(count.to_string());
    cmd.arg("-l").arg(bytes.to_string());
    cmd.arg("-w").arg(timeout.as_millis().to_string());
    cmd.arg(target);
    cmd
}

#[cfg(target_os = "windows")]
fn parse_output(output: &str) -> anyhow::Result<PingOutput> {
    let packet_loss_re = Regex::new(
        r"Packets: Sent = [0-9]+, Received = [0-9]+, Lost = [0-9]+ \(([0-9]+)% loss\),",
    )?;
    let rtt_re = Regex::new(r"Minimum = ([0-9]+)ms, Maximum = ([0-9]+)ms, Average = ([0-9]+)ms")?;
    parse_any_output(output, &packet_loss_re, &rtt_re)
}

fn parse_any_output(
    output: &str,
    packet_loss_re: &Regex,
    rtt_re: &Regex,
) -> anyhow::Result<PingOutput> {
    let mut packet_loss = 0.0;
    let mut rtt_min = 0.0;
    let mut rtt_avg = 0.0;
    let mut rtt_max = 0.0;

    for line in output.lines().map(str::trim) {
        if let Some(captures) = packet_loss_re.captures(line) {
            packet_loss = captures[1].parse()?;
        } else if let Some(captures) = rtt_re.captures(line) {
            rtt_min = captures[1].parse()?;
            rtt_avg = captures[2].parse()?;
            rtt_max = captures[3].parse()?;
        }
    }

    packet_loss /= 100.0;
    Ok(PingOutput {
        packet_loss,
        rtt_min,
        rtt_avg,
        rtt_max,
    })
}
