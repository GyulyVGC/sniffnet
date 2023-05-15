/// This enum represents the sniffing process status.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Status {
    /// Sniffnet has just been launched/restarted and gui is in the main screen.
    Init,
    /// The sniffing process is running: the application parses packets and periodically update the output report.
    Running,
}
