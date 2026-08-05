use std::fmt::Display;

/// Trait for logging errors in a unified way
pub trait ErrorLogger<T, E> {
    /// Log the error and its location
    #[allow(clippy::missing_errors_doc)]
    fn log_err(self, loc: Location) -> Result<T, E>;
}

impl<T, E: Display> ErrorLogger<T, E> for Result<T, E> {
    #[allow(clippy::print_stderr)]
    fn log_err(self, location: Location) -> Result<T, E> {
        if let Err(e) = &self {
            let file = location.file;
            let line = location.line;
            eprintln!("Sniffnet error at [{file}:{line}]: {e}");
            // in debug mode, panic on error
            #[cfg(debug_assertions)]
            #[allow(clippy::panic)]
            {
                panic!();
            }
        }

        self
    }
}

/// Log a non-fatal anomaly and its location.
///
/// Unlike [`ErrorLogger::log_err`], this never panics in debug builds. It's for
/// conditions caused by input Sniffnet doesn't control — a malformed datagram
/// arriving from the network, say — where aborting would hand whoever sent it a
/// way to kill the application.
#[allow(clippy::print_stderr)]
pub fn log_warning(location: &Location, message: &str) {
    let file = location.file;
    let line = location.line;
    eprintln!("Sniffnet warning at [{file}:{line}]: {message}");
}

/// Struct to store the location in the code (file and line)
pub struct Location {
    pub file: &'static str,
    pub line: u32,
}

#[macro_export]
/// Macro to get the current location in the code (file and line)
macro_rules! location {
    () => {
        Location {
            file: file!(),
            line: line!(),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_error_logger_panics_on_err_in_debug_mode() {
        let err_result: Result<usize, &str> = Err("test_error");
        let _err_handled = err_result.log_err(location!());
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn test_error_logger_no_panic_on_err_in_release_mode() {
        let err_result: Result<usize, &str> = Err("test_error");
        let err_handled = err_result.log_err(location!());
        assert_eq!(err_handled, err_result);
    }

    #[test]
    fn test_error_logger_ok() {
        let ok_result: Result<usize, &str> = Ok(2);
        let ok_handled = ok_result.log_err(location!());
        assert_eq!(ok_handled, ok_result);
    }
}
