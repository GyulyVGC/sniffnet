use std::ops::Sub;
use std::time::Duration;

pub struct TimingEvents {
    /// Instant of the last window focus
    focus: std::time::Instant,
    /// Instant of the last press on Copy IP button, with the related IP address
    copy_ip: (std::time::Instant, String),
    /// Instant of the last thumbnail mode enter
    thumbnail_enter: std::time::Instant,
    /// Instant of the last click on the thumbnail window
    thumbnail_click: std::time::Instant,
    /// Instant of the last edit of the notifications settings
    notifications_edit: std::time::Instant,
}

impl TimingEvents {
    const TIMEOUT_FOCUS: u64 = 200;
    const TIMEOUT_COPY_IP: u64 = 1500;
    const TIMEOUT_THUMBNAIL_ENTER: u64 = 1000;
    const TIMEOUT_THUMBNAIL_CLICK: u64 = 800;
    #[cfg(not(test))]
    const TIMEOUT_NOTIFICATIONS_EDIT: u64 = 3000;
    #[cfg(test)]
    pub const TIMEOUT_NOTIFICATIONS_EDIT: u64 = 100;

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

    pub fn thumbnail_enter_now(&mut self) {
        self.thumbnail_enter = std::time::Instant::now();
    }

    pub fn was_just_thumbnail_enter(&self) -> bool {
        self.thumbnail_enter.elapsed()
            < Duration::from_millis(TimingEvents::TIMEOUT_THUMBNAIL_ENTER)
    }

    pub fn thumbnail_click_now(&mut self) {
        self.thumbnail_click = std::time::Instant::now();
    }

    pub fn was_just_thumbnail_click(&self) -> bool {
        self.thumbnail_click.elapsed()
            < Duration::from_millis(TimingEvents::TIMEOUT_THUMBNAIL_CLICK)
    }

    pub fn notifications_edit_now(&mut self) {
        self.notifications_edit = std::time::Instant::now();
    }

    pub fn was_just_notifications_edit(&mut self) -> bool {
        self.notifications_edit.elapsed()
            < Duration::from_millis(TimingEvents::TIMEOUT_NOTIFICATIONS_EDIT)
    }
}

impl Default for TimingEvents {
    fn default() -> Self {
        Self {
            focus: std::time::Instant::now().sub(Duration::from_millis(400)),
            copy_ip: (std::time::Instant::now(), String::new()),
            thumbnail_enter: std::time::Instant::now(),
            thumbnail_click: std::time::Instant::now(),
            notifications_edit: std::time::Instant::now(),
        }
    }
}
