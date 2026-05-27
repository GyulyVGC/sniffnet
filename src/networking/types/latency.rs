use std::net::IpAddr;
use std::process::Command;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LatencyStatus {
    Measuring,
    Measured(Duration),
    Failed(String),
}

impl LatencyStatus {
    pub fn formatted(&self) -> String {
        match self {
            Self::Measuring => "Measuring...".to_string(),
            Self::Measured(latency) => format!("{} ms", latency.as_millis()),
            Self::Failed(error) => error.clone(),
        }
    }
}

pub async fn measure_latency(ip: IpAddr) -> (IpAddr, LatencyStatus) {
    (ip, measure_latency_inner(ip))
}

fn measure_latency_inner(ip: IpAddr) -> LatencyStatus {
    let ip = ip.to_string();
    let output = ping_command(&ip).output();

    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            parse_ping_latency(&stdout).map_or_else(
                || LatencyStatus::Failed("Latency unavailable".to_string()),
                LatencyStatus::Measured,
            )
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            LatencyStatus::Failed(if stderr.is_empty() {
                "Latency unavailable".to_string()
            } else {
                stderr
            })
        }
        Err(_) => LatencyStatus::Failed("Unable to run ping".to_string()),
    }
}

fn ping_command(ip: &str) -> Command {
    let mut command = Command::new("ping");

    #[cfg(target_os = "windows")]
    {
        command.args(["-n", "1", "-w", "1000", ip]);
    }

    #[cfg(target_os = "macos")]
    {
        command.args(["-c", "1", "-W", "1000", ip]);
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        command.args(["-c", "1", "-W", "1", ip]);
    }

    command
}

fn parse_ping_latency(output: &str) -> Option<Duration> {
    let time_pos = output.find("time")?;
    let after_time = &output[time_pos + "time".len()..];
    let value_start = after_time.find(['=', '<'])?;
    let value = after_time[value_start + 1..].trim_start();
    let value_end = value
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
        .unwrap_or(value.len());
    let millis = value[..value_end].parse::<f64>().ok()?;

    Some(Duration::from_secs_f64(millis / 1000.0))
}

#[cfg(test)]
mod tests {
    use super::parse_ping_latency;

    #[test]
    fn parses_linux_ping_latency() {
        let output = "64 bytes from 1.1.1.1: icmp_seq=1 ttl=58 time=14.2 ms";

        assert_eq!(parse_ping_latency(output).unwrap().as_millis(), 14);
    }

    #[test]
    fn parses_windows_ping_latency() {
        let output = "Reply from 1.1.1.1: bytes=32 time=23ms TTL=55";

        assert_eq!(parse_ping_latency(output).unwrap().as_millis(), 23);
    }

    #[test]
    fn parses_sub_millisecond_latency() {
        let output = "64 bytes from 127.0.0.1: icmp_seq=1 ttl=64 time<1 ms";

        assert_eq!(parse_ping_latency(output).unwrap().as_millis(), 1);
    }
}
