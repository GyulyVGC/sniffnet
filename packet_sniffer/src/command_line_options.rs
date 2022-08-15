use clap::Parser;

/// Struct for command line arguments, to be parsed using clap.
#[derive(Parser, Debug)]
pub struct Args {
    /// Name of the network adapter to be inspected, if omitted the default adapter is chosen.
    #[clap(short, long, value_parser, forbid_empty_values = true, default_value = "default")]
    pub adapter: String,

    /// Prints the list of available devices. Immediately terminates the program.
    #[clap(short, long)]
    pub device_list: bool,

    /// Sets the maximum port value to be considered, if omitted there is not ports higher bound.
    #[clap(short, long, value_parser, default_value_t = u16::MAX)]
    pub highest_port: u16,

    /// Sets the interval of time between report updates (value in seconds).
    #[clap(short, long, value_parser, default_value_t = 5)]
    pub interval: u64,

    /// Sets the minimum port value to be considered, if omitted there is not ports lower bound.
    #[clap(short, long, value_parser, default_value_t = u16::MIN)]
    pub lowest_port: u16,

    /// Sets the minimum value of transited packets for an address:port to be printed in the report.
    #[clap(short, long, value_parser, default_value_t = u32::MIN)]
    pub minimum_packets: u32,

    /// Filters packets on the basis of the IP version address (IPv4 or IPv6).
    #[clap(short, long, value_parser, default_value = "no filter")]
    pub network_layer_filter: String,

    /// Name of output file to contain the textual report, if omitted a default file is chosen.
    #[clap(short, long, value_parser, forbid_empty_values = true, default_value = "report.txt")]
    pub output_file: String,

    /// Filters packets on the basis of the transport layer protocol (TCP or UDP).
    #[clap(short, long, value_parser, default_value = "no filter")]
    pub transport_layer_filter: String,
}
