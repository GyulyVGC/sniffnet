use std::fmt::Display;

/// Trait for logging errors in a unified way
pub trait ErrorLogger<T, E> {
    /// Log the error and its location
    #[allow(clippy::missing_errors_doc)]
    fn log_err(self, loc: Location) -> Result<T, E>;
}

impl<T, E: Display> ErrorLogger<T, E> for Result<T, E> {
    fn log_err(self, location: Location) -> Result<T, E> {
        if let Err(e) = &self {
            let file = location.file;
            let line = location.line;
            eprintln!("Sniffnet error at [{file}:{line}]: {e}");
            // in debug mode, panic on error
            assert!(!cfg!(debug_assertions));
        }

        self
    }
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
