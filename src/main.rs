//! Multithreading application to intercept incoming and outgoing traffic through a user specified network interface of a computer.
//!
//! The application will periodically generate a human readable textual report,
//! providing statistics about the observed network packets divided by address:port pairs.
//!
//! The user can in any moment pause and resume the sniffing process.
//!
//! Packets can be filtered specifying command line options.
//!
//! Packets analysis is based on network and transport layers.
mod address_port;
mod report_info;
mod args;
mod thread_parse_packets_functions;
mod thread_write_report_functions;

use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use pcap::{Device};
use crate::address_port::{AddressPort};
use crate::report_info::{AppProtocol, ReportInfo, TransProtocol};
use crate::args::Args;
use crate::thread_parse_packets_functions::parse_packets_loop;
use crate::thread_write_report_functions::sleep_and_write_report_loop;
use clap::Parser;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use crossterm::{screen::RawScreen,  input::{input, InputEvent, KeyEvent}};
use colored;
use colored::Colorize;

/// This enum represents the sniffing process status.
#[derive(PartialEq, Eq)]
pub enum Status {
    /// The sniffing process is running: the application parses packets and periodically update the output report.
    Running,
    /// The sniffing process is pause by the user and waiting to be later resumed.
    Pause
}

/// Entry point of application execution.
///
/// It parses command line options using clap and generates two threads:
/// one to periodically update the report file and one to parse network packets.
///
/// The main thread will wait for user commands to pause, stop and resume the sniffing process.
fn main() {
    // enables cli colors
    colored::control::set_override(true);

    // parse arguments
    let args = Args::parse();
    let mut adapter: String = args.adapter;
    let output_file: String = args.output_file;
    let lowest_port = args.lowest_port;
    let highest_port = args.highest_port;
    let min_packets = args.minimum_packets;
    let interval = args.interval;
    let network_layer: String = args.network_layer_filter.to_ascii_lowercase();
    let network_layer_2: String = network_layer.clone();
    let transport_layer: String = args.transport_layer_filter.to_ascii_lowercase();
    let transport_layer_2: String = transport_layer.clone();

    if args.device_list == true {
        print_device_list();
        return;
    }

    if !is_valid_network_layer(network_layer.clone()) {
        eprint!("{}","\n\tERROR: Specified network layer filter must be equal to 'IPv4' or 'IPv6' (not case sensitive).\n\n".red());
        return;
    }

    if !is_valid_transport_layer(transport_layer.clone()) {
        eprint!("{}","\n\tERROR: Specified transport layer filter must be equal to 'TCP' or 'UDP' (not case sensitive).\n\n".red());
        return;
    }

    if lowest_port > highest_port {
        eprint!("{}", "\n\tERROR: Specified lowest port is greater than specified highest port.\n\n".red());
        return;
    }

    if interval == 0 {
        eprint!("{}", "\n\tERROR: Specified time interval is null.\n\n".red());
        return;
    }

    let found_device_option = retrieve_device(&mut adapter);

    if found_device_option.is_none() {
        eprint!("{}", "\n\tERROR: Specified network adapter does not exist. Use option '-d' to list all the available devices.\n\n".red());
        return;
    }

    let found_device = found_device_option.unwrap();
    let device_name = found_device.clone().name;

    let mutex_map1 = Arc::new(Mutex::new(HashMap::new()));
    let mutex_map2 = mutex_map1.clone();
    let status_pair1 = Arc::new((Mutex::new(Status::Running), Condvar::new()));
    let status_pair2 = status_pair1.clone();
    let status_pair3 = status_pair1.clone();

    println!("{}{}{}", "\n\tSniffing network adapter '".bright_blue(), device_name.bright_blue(), "'".bright_blue());
    println!("{}{}{}{}{}", "\tUpdating the file '".bright_blue(), output_file.bright_blue(),
              "' every ".bright_blue(), interval.to_string().bright_blue(), " seconds".bright_blue());
    println!("{}{}{}{}", "\n\tPress the key".bright_blue(),  "\n\t\t- 'p' to pause".yellow(),
             "\n\t\t- 's' to stop".red(), "\n\tthe application\n".bright_blue());

    // Thread 1: updates textual report
    thread::spawn(move || {
        sleep_and_write_report_loop(lowest_port, highest_port, interval, min_packets,
                                    device_name, network_layer,
                                    transport_layer, output_file,
                                    mutex_map2, status_pair3);
    });

    // Thread 2: parses packets
    thread::spawn(move || {
        parse_packets_loop(found_device, lowest_port, highest_port, network_layer_2,
                           transport_layer_2, mutex_map1, status_pair1);
    });

    // Main thread: updates application status
    set_status_by_key(status_pair2);

}


/// Prints the list of available network adapters' names and addresses.
///
/// This function is called if the user specifies the ```-d``` command line option.
fn print_device_list() {
    println!();
    for dev in Device::list().expect("Error retrieving device list\n") {
        print!("{}{}{}", "\tDevice: ".bright_blue(), dev.name.bright_blue(),
            "\n\t\tAddresses: ".bright_blue());
        if dev.addresses.len() == 0 {
            println!();
        }
        for addr in dev.addresses {
            let address_string = addr.addr.to_string();
            print!("{}\n\t\t\t   ", address_string.bright_blue());
        }
        println!();
    }
    println!();
}


/// Given the name of the desired network adapter, this function returns an ```Option<Device>```
/// which contains the corresponding ```Device``` struct if the provided network adapter exists or
/// a ```None``` value if it doesn't exist.
///
/// # Arguments
///
/// * `adapter` - A String representing the name of the network adapter to be sniffed.
fn retrieve_device(adapter: &mut String) -> Option<Device> {
    let mut found_device = None;
    if (*adapter).eq(&"default".to_string()) {
        *adapter = Device::lookup().expect("Error retrieving default network adapter\n").name;
    }
    let dev_list = Device::list().expect("Unable to retrieve network adapters list\n");
    for device in dev_list {
        if device.name == *adapter {
            found_device = Some(device);
            break;
        }
    }
    return found_device;
}


/// Checks if the provided ```network_layer``` equals "ipv6" or "ipv4" or "no filter".
///
/// # Arguments
///
/// * `network_layer` - A String representing the IP version to be filtered. Specified by the user through the
/// ```-n``` option.
///
/// # Examples
///
/// ```
/// let x = is_valid_network_layer("ipv7");
/// assert_eq!(x, false);
///
/// let y = is_valid_network_layer("ipv6");
/// assert_eq!(y, true)
/// ```
fn is_valid_network_layer(network_layer: String) -> bool {
    network_layer.cmp(&"ipv6".to_string()) == Equal
        || network_layer.cmp(&"ipv4".to_string()) == Equal
        || network_layer.cmp(&"no filter".to_string()) == Equal
}


/// Checks if the provided ```transport_layer``` equals "tcp" or "udp" or "no filter".
///
/// # Arguments
///
/// * `transport_layer` - A String representing the transport protocol to be filtered. Specified by the user through the
/// ```-t``` option.
///
/// # Examples
///
/// ```
/// let x = is_valid_transport_layer("http");
/// assert_eq!(x, false);
///
/// let y = is_valid_transport_layer("tcp");
/// assert_eq!(y, true)
/// ```
fn is_valid_transport_layer(transport_layer: String) -> bool {
    transport_layer.cmp(&"tcp".to_string()) == Equal
        || transport_layer.cmp(&"udp".to_string()) == Equal
        || transport_layer.cmp(&"no filter".to_string()) == Equal
}


/// Loop waiting for command line inputs by the user. Used to pause, resume and stop the sniffing process.
///
/// If the 'p' character is received, the sniffing process is paused.
///
/// If the 'r' character is received, the sniffing process is resumed.
///
/// If the 's' character is received, the sniffing process is stopped.
///
/// # Arguments
///
/// * `status_pair` - Shared variable to change the application current status.
fn set_status_by_key(status_pair: Arc<(Mutex<Status>, Condvar)>) {

    let _raw = RawScreen::into_raw_mode();
    let mut reader = input().read_sync();
    let cvar = &status_pair.1;
    loop {
        if let Some(event) = reader.next() { // Blocking call
            let mut status = status_pair.0.lock().expect("Error acquiring mutex\n");
            match event {
                InputEvent::Keyboard(KeyEvent::Char('p')) => {
                    if *status == Status::Running {
                        println!("\t{}", "Sniffnet paused... Press 'r' to resume\r".yellow());
                        *status = Status::Pause;
                    }
                }
                InputEvent::Keyboard(KeyEvent::Char('r')) => {
                    if *status == Status::Pause {
                        println!("\t{}", "Sniffnet resumed\r".green());
                        *status = Status::Running;
                        cvar.notify_all();
                    }
                }
                InputEvent::Keyboard(KeyEvent::Char('s')) => {
                    println!("\n\t{}", "Sniffnet stopped\n\r".red());
                    return;
                }
                _ => { /* Other events */ }
            }
        }
    }
}