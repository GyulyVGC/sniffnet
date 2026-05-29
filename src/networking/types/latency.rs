use std::io;
use std::net::IpAddr;
use std::process::{Command, Output};
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
    measure_latency_with_executor(ip, &SystemPingExecutor)
}

fn measure_latency_with_executor(ip: IpAddr, executor: &impl PingExecutor) -> LatencyStatus {
    let ip = ip.to_string();
    let mut command = ping_command(&ip);
    let output = executor.execute(&mut command);

    match output {
        Ok(output) if output.success => {
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

struct PingOutput {
    success: bool,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl From<Output> for PingOutput {
    fn from(output: Output) -> Self {
        Self {
            success: output.status.success(),
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }
}

trait PingExecutor {
    fn execute(&self, command: &mut Command) -> io::Result<PingOutput>;
}

struct SystemPingExecutor;

impl PingExecutor for SystemPingExecutor {
    fn execute(&self, command: &mut Command) -> io::Result<PingOutput> {
        command.output().map(PingOutput::from)
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
    use std::cell::RefCell;
    use std::ffi::OsString;
    use std::io;
    use std::net::{IpAddr, Ipv4Addr};
    use std::process::Command;
    use std::time::Duration;

    use super::{
        LatencyStatus, PingExecutor, PingOutput, measure_latency_with_executor, parse_ping_latency,
        ping_command,
    };

    struct FakePingExecutor {
        output: RefCell<Option<io::Result<PingOutput>>>,
        observed_command: RefCell<Option<(OsString, Vec<OsString>)>>,
    }

    impl FakePingExecutor {
        fn new(output: io::Result<PingOutput>) -> Self {
            Self {
                output: RefCell::new(Some(output)),
                observed_command: RefCell::new(None),
            }
        }

        fn observed_command(&self) -> (String, Vec<String>) {
            let (program, args) = self
                .observed_command
                .borrow()
                .as_ref()
                .expect("ping command should be executed")
                .clone();

            (
                program.to_string_lossy().into_owned(),
                args.into_iter()
                    .map(|arg| arg.to_string_lossy().into_owned())
                    .collect(),
            )
        }
    }

    impl PingExecutor for FakePingExecutor {
        fn execute(&self, command: &mut Command) -> io::Result<PingOutput> {
            self.observed_command.replace(Some((
                command.get_program().to_os_string(),
                command.get_args().map(|arg| arg.to_os_string()).collect(),
            )));

            self.output
                .borrow_mut()
                .take()
                .expect("fake ping output should only be consumed once")
        }
    }

    fn successful_ping(stdout: &str) -> io::Result<PingOutput> {
        Ok(PingOutput {
            success: true,
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
        })
    }

    fn failed_ping(stderr: &str) -> io::Result<PingOutput> {
        Ok(PingOutput {
            success: false,
            stdout: Vec::new(),
            stderr: stderr.as_bytes().to_vec(),
        })
    }

    fn command_args(command: &Command) -> Vec<String> {
        command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect()
    }

    #[cfg(target_os = "windows")]
    fn expected_ping_args(ip: &str) -> Vec<String> {
        ["-n", "1", "-w", "1000", ip]
            .into_iter()
            .map(String::from)
            .collect()
    }

    #[cfg(target_os = "macos")]
    fn expected_ping_args(ip: &str) -> Vec<String> {
        ["-c", "1", "-W", "1000", ip]
            .into_iter()
            .map(String::from)
            .collect()
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    fn expected_ping_args(ip: &str) -> Vec<String> {
        ["-c", "1", "-W", "1", ip]
            .into_iter()
            .map(String::from)
            .collect()
    }

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

    #[test]
    fn builds_ping_command_for_current_platform() {
        let command = ping_command("1.1.1.1");

        assert_eq!(command.get_program().to_string_lossy(), "ping");
        assert_eq!(command_args(&command), expected_ping_args("1.1.1.1"));
    }

    #[test]
    fn executes_ping_command_and_parses_latency() {
        let executor = FakePingExecutor::new(successful_ping(
            "64 bytes from 1.1.1.1: icmp_seq=1 ttl=58 time=14.2 ms",
        ));

        let status =
            measure_latency_with_executor(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), &executor);

        assert_eq!(
            status,
            LatencyStatus::Measured(Duration::from_micros(14_200))
        );
        assert_eq!(
            executor.observed_command(),
            ("ping".to_string(), expected_ping_args("1.1.1.1"))
        );
    }

    #[test]
    fn reports_unavailable_latency_when_ping_output_has_no_latency() {
        let executor = FakePingExecutor::new(successful_ping("1 packets transmitted, 0 received"));

        let status =
            measure_latency_with_executor(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), &executor);

        assert_eq!(
            status,
            LatencyStatus::Failed("Latency unavailable".to_string())
        );
    }

    #[test]
    fn reports_ping_stderr_when_command_fails() {
        let executor = FakePingExecutor::new(failed_ping("Request timeout for icmp_seq 0\n"));

        let status =
            measure_latency_with_executor(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), &executor);

        assert_eq!(
            status,
            LatencyStatus::Failed("Request timeout for icmp_seq 0".to_string())
        );
    }

    #[test]
    fn reports_unavailable_latency_when_failed_ping_has_no_stderr() {
        let executor = FakePingExecutor::new(failed_ping(""));

        let status =
            measure_latency_with_executor(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), &executor);

        assert_eq!(
            status,
            LatencyStatus::Failed("Latency unavailable".to_string())
        );
    }

    #[test]
    fn reports_launch_error_when_ping_cannot_start() {
        let executor = FakePingExecutor::new(Err(io::Error::new(
            io::ErrorKind::NotFound,
            "ping not found",
        )));

        let status =
            measure_latency_with_executor(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), &executor);

        assert_eq!(
            status,
            LatencyStatus::Failed("Unable to run ping".to_string())
        );
    }
}
