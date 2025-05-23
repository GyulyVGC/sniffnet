use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::IpAddr;
use std::sync::Arc;
use std::fs;
use std::io::{BufRead, BufReader};

#[cfg(not(test))]
use crate::utils::error_logger::{ErrorLogger, Location};
#[cfg(not(test))]
use crate::{location, SNIFFNET_LOWERCASE};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConfigBlacklist {
    pub blacklist_path: Option<String>,
    #[serde(skip)]
    pub loaded_ips: Option<Arc<HashSet<IpAddr>>>,
}

impl ConfigBlacklist {
    #[cfg(not(test))]
    pub const FILE_NAME: &'static str = "blacklist";

    /// Load IP addresses from a blacklist file with validation and error handling.
    /// 
    /// This function reads a text file containing IP addresses (one per line) and returns
    /// a HashSet of parsed IP addresses. The function provides the following features:
    /// 
    /// - Support for comments (lines starting with '#')
    /// - Support for empty lines
    /// - Line-by-line error reporting for invalid IP addresses
    /// - Both IPv4 and IPv6 addresses are supported
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the blacklist file
    /// 
    /// # Returns
    /// 
    /// * `Ok(HashSet<IpAddr>)` - Successfully loaded IP addresses
    /// * `Err(String)` - Error message describing what went wrong
    /// 
    /// # Example file format
    /// 
    /// ```text
    /// # This is a comment
    /// 192.168.1.100
    /// 10.0.0.1
    /// 
    /// # IPv6 addresses are also supported
    /// 2001:db8::1
    /// ```
    pub fn load_ips_from_file(path: &str) -> Result<HashSet<IpAddr>, String> {
        // Check if file exists
        let _metadata = fs::metadata(path)
            .map_err(|e| format!("Failed to access blacklist file '{}': {}", path, e))?;

        let file = fs::File::open(path)
            .map_err(|e| format!("Failed to open blacklist file '{}': {}", path, e))?;
        
        let mut ips = HashSet::new();
        let reader = BufReader::new(file);
        let mut line_number = 0;
        let mut parse_errors = Vec::new();
        
        for line_result in reader.lines() {
            line_number += 1;
            
            let line = line_result
                .map_err(|e| format!("Failed to read line {} from '{}': {}", line_number, path, e))?;
            
            let trimmed = line.trim();
            
            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            
            // Try to parse the IP address
            match trimmed.parse::<IpAddr>() {
                Ok(ip_addr) => {
                    ips.insert(ip_addr);
                }
                Err(_) => {
                    parse_errors.push(format!("Line {}: Invalid IP address '{}'", line_number, trimmed));
                }
            }
        }
        
        // Report parse errors if any
        if !parse_errors.is_empty() {
            eprintln!("Blacklist parsing warnings for '{}':", path);
            for error in parse_errors.iter().take(10) {
                eprintln!("  {}", error);
            }
            if parse_errors.len() > 10 {
                eprintln!("  ... and {} more errors", parse_errors.len() - 10);
            }
        }
        
        Ok(ips)
    }

    #[cfg(not(test))]
    pub fn load() -> Self {
        if let Ok(blacklist_config) =
            confy::load::<ConfigBlacklist>(SNIFFNET_LOWERCASE, Self::FILE_NAME)
        {
            let mut loaded_config = blacklist_config;
            // Attempt to load IPs if path is set
            if let Some(path) = &loaded_config.blacklist_path {
                match Self::load_ips_from_file(path) {
                    Ok(ips) => {
                        eprintln!("Successfully loaded {} IP addresses from blacklist '{}'", ips.len(), path);
                        loaded_config.loaded_ips = Some(Arc::new(ips));
                    }
                    Err(e) => {
                        eprintln!("Failed to load blacklist: {}", e);
                        loaded_config.loaded_ips = None;
                    }
                }
            }
            loaded_config
        } else {
            let default_config = ConfigBlacklist::default();
            let _ = confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, default_config.clone())
                .log_err(location!());
            default_config
        }
    }

    #[cfg(test)]
    pub fn load() -> Self {
        ConfigBlacklist::default()
    }

    #[cfg(not(test))]
    pub fn store(&self) {
        let _ = confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, self).log_err(location!());
    }

    #[cfg(test)]
    pub fn store(&self) {
        // No-op for test
    }
}

impl Default for ConfigBlacklist {
    fn default() -> Self {
        ConfigBlacklist {
            blacklist_path: None,
            loaded_ips: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_ips_from_valid_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "192.168.1.1").unwrap();
        writeln!(temp_file, "10.0.0.1").unwrap();
        writeln!(temp_file, "2001:db8::1").unwrap();
        
        let result = ConfigBlacklist::load_ips_from_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        
        let ips = result.unwrap();
        assert_eq!(ips.len(), 3);
        assert!(ips.contains(&"192.168.1.1".parse().unwrap()));
        assert!(ips.contains(&"10.0.0.1".parse().unwrap()));
        assert!(ips.contains(&"2001:db8::1".parse().unwrap()));
    }

    #[test]
    fn test_load_ips_with_comments_and_empty_lines() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "# This is a comment").unwrap();
        writeln!(temp_file, "192.168.1.1").unwrap();
        writeln!(temp_file, "").unwrap();
        writeln!(temp_file, "  # Another comment with spaces").unwrap();
        writeln!(temp_file, "10.0.0.1").unwrap();
        writeln!(temp_file, "   ").unwrap(); // Line with only spaces
        
        let result = ConfigBlacklist::load_ips_from_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        
        let ips = result.unwrap();
        assert_eq!(ips.len(), 2);
        assert!(ips.contains(&"192.168.1.1".parse().unwrap()));
        assert!(ips.contains(&"10.0.0.1".parse().unwrap()));
    }

    #[test]
    fn test_load_ips_with_invalid_entries() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "192.168.1.1").unwrap();
        writeln!(temp_file, "invalid_ip").unwrap();
        writeln!(temp_file, "10.0.0.1").unwrap();
        writeln!(temp_file, "999.999.999.999").unwrap();
        writeln!(temp_file, "192.168.1.2").unwrap();
        
        let result = ConfigBlacklist::load_ips_from_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        
        let ips = result.unwrap();
        assert_eq!(ips.len(), 3); // Only valid IPs should be loaded
        assert!(ips.contains(&"192.168.1.1".parse().unwrap()));
        assert!(ips.contains(&"10.0.0.1".parse().unwrap()));
        assert!(ips.contains(&"192.168.1.2".parse().unwrap()));
    }

    #[test]
    fn test_load_ips_from_nonexistent_file() {
        let result = ConfigBlacklist::load_ips_from_file("/nonexistent/path/to/file.txt");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to access blacklist file"));
    }

    #[test]
    fn test_load_ips_empty_file() {
        let temp_file = NamedTempFile::new().unwrap();
        
        let result = ConfigBlacklist::load_ips_from_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        
        let ips = result.unwrap();
        assert_eq!(ips.len(), 0);
    }

    #[test]
    fn test_load_ips_whitespace_trimming() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "  192.168.1.1  ").unwrap();
        writeln!(temp_file, "\t10.0.0.1\t").unwrap();
        writeln!(temp_file, " 172.16.0.1 ").unwrap();
        
        let result = ConfigBlacklist::load_ips_from_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        
        let ips = result.unwrap();
        assert_eq!(ips.len(), 3);
        assert!(ips.contains(&"192.168.1.1".parse().unwrap()));
        assert!(ips.contains(&"10.0.0.1".parse().unwrap()));
        assert!(ips.contains(&"172.16.0.1".parse().unwrap()));
    }

    #[test]
    fn test_load_ips_duplicate_entries() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "192.168.1.1").unwrap();
        writeln!(temp_file, "192.168.1.1").unwrap(); // Duplicate
        writeln!(temp_file, "10.0.0.1").unwrap();
        
        let result = ConfigBlacklist::load_ips_from_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        
        let ips = result.unwrap();
        assert_eq!(ips.len(), 2); // HashSet should deduplicate
        assert!(ips.contains(&"192.168.1.1".parse().unwrap()));
        assert!(ips.contains(&"10.0.0.1".parse().unwrap()));
    }

    #[test]
    fn test_config_blacklist_default() {
        let config = ConfigBlacklist::default();
        assert_eq!(config.blacklist_path, None);
        assert_eq!(config.loaded_ips, None);
    }

    #[test]
    fn test_load_ips_mixed_ipv4_ipv6() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "192.168.1.1").unwrap();
        writeln!(temp_file, "::1").unwrap();
        writeln!(temp_file, "127.0.0.1").unwrap();
        writeln!(temp_file, "2001:db8::8a2e:370:7334").unwrap();
        writeln!(temp_file, "fe80::1").unwrap();
        
        let result = ConfigBlacklist::load_ips_from_file(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        
        let ips = result.unwrap();
        assert_eq!(ips.len(), 5);
        assert!(ips.contains(&"192.168.1.1".parse().unwrap()));
        assert!(ips.contains(&"::1".parse().unwrap()));
        assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
        assert!(ips.contains(&"2001:db8::8a2e:370:7334".parse().unwrap()));
        assert!(ips.contains(&"fe80::1".parse().unwrap()));
    }
} 