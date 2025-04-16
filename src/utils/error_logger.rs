use std::fmt::Display;

/// Trait for logging errors in a unified way
pub trait ErrorLogger<T, E> {
    /// Log the error and its location
    #[allow(clippy::missing_errors_doc)]
    fn log_err(self, loc: Location) -> Result<T, E>;
}

impl<T, E: Display> ErrorLogger<T, E> for Result<T, E> {
    fn log_err(self, location: Location) -> Result<T, E> {
        self.inspect_err(|e| {
            eprintln!("[{}:{}] {e}", location.file, location.line);
        })
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
    fn test_error_logger() {
        let err_result: Result<usize, &str> = Err("test_error");
        let err_handled = err_result.log_err(location!());
        assert_eq!(err_handled, err_result);

        let ok_result: Result<usize, &str> = Ok(2);
        let ok_handled = ok_result.log_err(location!());
        assert_eq!(ok_handled, ok_result);
    }
}
