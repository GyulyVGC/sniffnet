pub struct ReportInfo {
    transmitted_bytes: u16,
    initial_timestamp: String,
    final_timestamp: String,
}

impl ReportInfo {

    pub fn new (address: String, port: String) -> Self {
        ReportInfo {
            transmitted_bytes: 0,
            initial_timestamp: "".to_string(),
            final_timestamp: "".to_string()
        }
    }

}