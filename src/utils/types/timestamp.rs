#[derive(Clone, Default, Debug, Copy, Eq, PartialEq)]
pub struct Timestamp {
    secs: i64,
    usecs: i64,
}

impl Timestamp {
    pub fn new(secs: i64, usecs: i64) -> Self {
        Self { secs, usecs }
    }

    pub fn secs(&self) -> i64 {
        self.secs
    }

    pub fn to_usecs(self) -> Option<i64> {
        self.secs
            .checked_mul(1_000_000)
            .and_then(|x| x.checked_add(self.usecs))
    }

    pub fn add_secs(&mut self, secs: i64) {
        self.secs += secs;
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_usecs().cmp(&other.to_usecs())
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_cmp() {
        let t0 = Timestamp::new(-1, 700);
        let t1 = Timestamp::new(1, 500);
        let t2 = Timestamp::new(1, 600);
        let t3 = Timestamp::new(2, 0);

        assert_eq!(t0.cmp(&t1), std::cmp::Ordering::Less);
        assert_eq!(t1.cmp(&t0), std::cmp::Ordering::Greater);
        assert_eq!(t0.cmp(&t2), std::cmp::Ordering::Less);
        assert_eq!(t2.cmp(&t0), std::cmp::Ordering::Greater);
        assert_eq!(t0.cmp(&t3), std::cmp::Ordering::Less);
        assert_eq!(t3.cmp(&t0), std::cmp::Ordering::Greater);
        assert_eq!(t1.cmp(&t2), std::cmp::Ordering::Less);
        assert_eq!(t2.cmp(&t1), std::cmp::Ordering::Greater);
        assert_eq!(t1.cmp(&t3), std::cmp::Ordering::Less);
        assert_eq!(t3.cmp(&t1), std::cmp::Ordering::Greater);
        assert_eq!(t2.cmp(&t3), std::cmp::Ordering::Less);
        assert_eq!(t3.cmp(&t2), std::cmp::Ordering::Greater);

        assert_eq!(t0.cmp(&t0), std::cmp::Ordering::Equal);
        assert_eq!(t1.cmp(&t1), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_timestamp_to_usecs() {
        let t = Timestamp::new(-1, -700);
        assert_eq!(t.to_usecs(), Some(-1_000_700));
        let t = Timestamp::new(137, 500);
        assert_eq!(t.to_usecs(), Some(137_000_500));
        let t = Timestamp::new(0, i64::MAX);
        assert_eq!(t.to_usecs(), Some(i64::MAX));
        let t = Timestamp::new(0, i64::MIN);
        assert_eq!(t.to_usecs(), Some(i64::MIN));

        let t = Timestamp::new(i64::MAX, 0);
        assert_eq!(t.to_usecs(), None);
        let t = Timestamp::new(i64::MIN, 0);
        assert_eq!(t.to_usecs(), None);
        let t = Timestamp::new(1, i64::MAX);
        assert_eq!(t.to_usecs(), None);
        let t = Timestamp::new(-1, i64::MIN);
        assert_eq!(t.to_usecs(), None);
        let t = Timestamp::new(1, i64::MIN);
        assert!(t.to_usecs().is_some());
    }
}
