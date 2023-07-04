/// This enum represents the sniffing process status.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Status {
    /// Sniffnet has just been launched/restarted and gui is in the main screen.
    Init,
    /// The sniffing process is running: the application parses packets and periodically update the output report.
    Running,
}

/// A struct to record last focus status of the window.
pub struct FocusState {
    /// The time of the window be focused on
    last_focus_time: std::time::Instant,
    /// A fixed short period of time to determine
    /// if we focused on the window just now.
    just_focus_timeout: std::time::Duration,
}

impl FocusState {
    pub fn new(focus_timeout: u64) -> Self {
        Self {
            last_focus_time: std::time::Instant::now(),
            just_focus_timeout: std::time::Duration::from_millis(focus_timeout),
        }
    }

    /// Update last focus time to current time
    pub fn update(&mut self) {
        self.last_focus_time = std::time::Instant::now();
    }

    /// Check if the window be focused just now
    pub fn is_just_focus(&self) -> bool {
        self.last_focus_time.elapsed() < self.just_focus_timeout
    }
}

