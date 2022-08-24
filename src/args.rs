//! Module defining the `Args` struct used for command line options.

use clap::Parser;

/// Struct for command line arguments, to be parsed using clap.
#[derive(Parser, Debug)]
pub struct Args {
    /// Name of the network adapter to be inspected, if omitted the default adapter is chosen.
    ///
    /// If a non-existing adapter is provided, the application raises an error and terminates.
    ///
    /// This option must be followed by a textual value.
    #[clap(short, long, value_parser, forbid_empty_values = true, default_value = "default")]
    pub adapter: String,

    /// Filters packets on the basis of the application layer protocol.
    ///
    /// This option must be followed by a textual value.
    #[clap(long="app", value_parser, default_value = "no filter")]
    pub application_layer_filter: String,

    /// Prints list of the available network interfaces.
    ///
    /// Immediately terminates the program.
    ///
    /// This option does not need to be followed by a value.
    #[clap(short, long)]
    pub device_list: bool,

    /// Sets the maximum port value to be considered, if omitted there is not ports higher bound.
    ///
    /// If the highest-port provided value is lower than the lowest-port provided value, the application raises an error and terminates.
    ///
    /// This option must be followed by an integer value between 0 and 65535.
    #[clap(short, long, value_parser, default_value_t = u16::MAX)]
    pub highest_port: u16,

    /// Sets the interval of time between report updates (value in seconds).
    ///
    /// This option must be followed by a positive integer value.
    #[clap(short, long, value_parser, default_value_t = 5)]
    pub interval: u64,

    /// Sets the lowest port value to be considered, if omitted there is not ports lower bound.
    ///
    /// If the lowest-port provided value is lower than the highest-port provided value, the application raises an error and terminates.
    ///
    /// This option must be followed by an integer value between 0 and 65535.
    #[clap(short, long, value_parser, default_value_t = u16::MIN)]
    pub lowest_port: u16,

    /// Sets the minimum value of transited packets for an address:port to be printed in the report.
    ///
    /// This option must be followed by a positive integer value.
    #[clap(short, long, value_parser, default_value_t = u32::MIN)]
    pub minimum_packets: u32,

    /// Filters packets on the basis of the IP version address (IPv4 or IPv6).
    ///
    /// This option must be followed by a textual value.
    #[clap(short, long="net", value_parser, default_value = "no filter")]
    pub network_layer_filter: String,

    /// Name of output file to contain the textual report, if omitted a default file is chosen.
    ///
    /// This option must be followed by a textual value.
    #[clap(short, long, value_parser, forbid_empty_values = true, default_value = "sniffnet_report.txt")]
    pub output_file: String,

    /// Filters packets on the basis of the transport layer protocol (TCP or UDP).
    ///
    /// This option must be followed by a textual value.
    #[clap(short, long="trans", value_parser, default_value = "no filter")]
    pub transport_layer_filter: String,
}
