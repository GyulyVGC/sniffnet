use std::time::Duration;

pub struct TimingEvents {
    /// Timestamp of last window focus
    pub focus: std::time::Instant,
    /// Timestamp of the last press on Copy IP button, with the related IP address
    pub copy_ip: (std::time::Instant, String),
}

impl TimingEvents {
    const TIMEOUT_FOCUS: u64 = 200;
    const TIMEOUT_COPY_IP: u64 = 1500;

    pub fn focus_now(&mut self) {
        self.focus = std::time::Instant::now();
    }

    pub fn was_just_focus(&self) -> bool {
        self.focus.elapsed() < Duration::from_millis(TimingEvents::TIMEOUT_FOCUS)
    }

    pub fn copy_ip_now(&mut self, ip: String) {
        self.copy_ip = (std::time::Instant::now(), ip);
    }

    pub fn was_just_copy_ip(&self, ip: &String) -> bool {
        self.copy_ip.0.elapsed() < Duration::from_millis(TimingEvents::TIMEOUT_COPY_IP)
            && self.copy_ip.1.eq(ip)
    }
}

impl Default for TimingEvents {
    fn default() -> Self {
        Self {
            focus: std::time::Instant::now(),
            copy_ip: (std::time::Instant::now(), String::new()),
        }
    }
}
