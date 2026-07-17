use std::net::IpAddr;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use surge_ping::{Client, Config, ICMP, PingIdentifier, PingSequence, SurgeError};

const PING_TIMEOUT: Duration = Duration::from_secs(1);
const PING_PAYLOAD: [u8; 8] = [0; 8];
const PING_COUNT: usize = 3;

static IPV4_CLIENT: OnceLock<Arc<Client>> = OnceLock::new();
static IPV6_CLIENT: OnceLock<Arc<Client>> = OnceLock::new();
static PING_SEQUENCE: AtomicU16 = AtomicU16::new(0);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LatencyStatus {
    Measuring,
    Measured(Duration),
    Failed(String),
}

pub async fn measure_latency(ip: IpAddr) -> (IpAddr, LatencyStatus) {
    (ip, measure_latency_inner(ip).await)
}

async fn measure_latency_inner(ip: IpAddr) -> LatencyStatus {
    let client = match client_for(ip) {
        Ok(client) => client,
        Err(error) => return LatencyStatus::Failed(error),
    };

    let mut pinger = client.pinger(ip, ping_identifier()).await;
    pinger.timeout(PING_TIMEOUT);

    let mut sum = Duration::ZERO;
    let mut received: u32 = 0;
    let mut last_error = None;
    for _ in 0..PING_COUNT {
        match pinger.ping(next_sequence(), &PING_PAYLOAD).await {
            Ok((_packet, latency)) => {
                sum += latency;
                received += 1;
            }
            Err(error @ SurgeError::Timeout { .. }) => last_error = Some(error.to_string()),
            Err(error) => {
                last_error = Some(error.to_string());
                break;
            }
        }
    }

    match received {
        0 => LatencyStatus::Failed(last_error.unwrap_or_else(|| "no reply".to_string())),
        n => LatencyStatus::Measured(sum / n),
    }
}

fn client_for(ip: IpAddr) -> Result<Arc<Client>, String> {
    let (cell, kind) = match ip {
        IpAddr::V4(_) => (&IPV4_CLIENT, ICMP::V4),
        IpAddr::V6(_) => (&IPV6_CLIENT, ICMP::V6),
    };

    if let Some(client) = cell.get() {
        return Ok(Arc::clone(client));
    }

    let client = latency_client(kind)?;
    Ok(Arc::clone(cell.get_or_init(|| client)))
}

fn latency_client(kind: ICMP) -> Result<Arc<Client>, String> {
    let config = match kind {
        ICMP::V4 => Config::default(),
        ICMP::V6 => Config::builder().kind(ICMP::V6).build(),
    };

    Client::new(&config)
        .map(Arc::new)
        .map_err(|error| error.to_string())
}

fn ping_identifier() -> PingIdentifier {
    PingIdentifier(std::process::id() as u16)
}

fn next_sequence() -> PingSequence {
    PingSequence(PING_SEQUENCE.fetch_add(1, Ordering::Relaxed))
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use super::{LatencyStatus, measure_latency_inner, next_sequence, ping_identifier};

    #[test]
    fn uses_process_id_as_ping_identifier() {
        assert_eq!(ping_identifier().0, std::process::id() as u16);
    }

    #[test]
    fn increments_ping_sequence() {
        let first = next_sequence().0;
        let second = next_sequence().0;

        assert_eq!(second, first.wrapping_add(1));
    }

    #[tokio::test]
    async fn measures_ipv4_loopback_latency_end_to_end() {
        let status = measure_latency_inner(IpAddr::V4(Ipv4Addr::LOCALHOST)).await;

        assert_measured_or_local_permission_error(status);
    }

    #[tokio::test]
    async fn measures_ipv6_loopback_latency_end_to_end() {
        let status = measure_latency_inner(IpAddr::V6(Ipv6Addr::LOCALHOST)).await;

        assert_measured_or_local_permission_error(status);
    }

    fn assert_measured_or_local_permission_error(status: LatencyStatus) {
        match status {
            LatencyStatus::Measured(_) => {}
            LatencyStatus::Failed(error)
                if env::var_os("CI").is_none()
                    && (error.contains("Operation not permitted")
                        || error.contains("Permission denied")) => {}
            LatencyStatus::Failed(error) => {
                panic!("expected loopback latency, got error: {error}");
            }
            LatencyStatus::Measuring => {
                panic!("expected completed loopback latency");
            }
        }
    }
}
