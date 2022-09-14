//#![feature(test)]

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
mod address_port_pair;
mod info_address_port_pair;
mod args;
mod thread_parse_packets_functions;
mod thread_write_report_functions;
mod info_traffic;

use std::cmp::Ordering::Equal;
use pcap::{Device};
use crate::info_address_port_pair::{AppProtocol, TransProtocol};
use crate::args::Args;
use crate::thread_parse_packets_functions::parse_packets_loop;
use crate::thread_write_report_functions::sleep_and_write_report_loop;
use clap::Parser;
use std::{io, panic, process, thread};
use std::io::Write;
use std::sync::{Arc, Mutex, Condvar};
use crossterm::{screen::RawScreen,  input::{input, InputEvent, KeyEvent}};
use colored::Colorize;
use crate::info_traffic::InfoTraffic;

/// This enum represents the sniffing process status.
#[derive(PartialEq, Eq)]
pub enum Status {
    /// The sniffing process is running: the application parses packets and periodically update the output report.
    Running,
    /// The sniffing process is pause by the user and waiting to be later resumed.
    Pause,
    /// The sniffing process is killed.
    Stop
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
    #[cfg(target_os = "windows")]
    colored::control::set_virtual_terminal(true).unwrap();

    // parse arguments
    let args = Args::parse();
    let mut adapter: String = args.adapter;
    let output_folder: String = args.output_folder;
    let lowest_port = args.lowest_port;
    let highest_port = args.highest_port;
    let min_packets = args.minimum_packets;
    let interval: u64 = args.interval;
    let network_layer: String = args.network_layer_filter.to_ascii_lowercase();
    let network_layer_2: String = network_layer.clone();
    let transport_layer: String = args.transport_layer_filter.to_ascii_lowercase();
    let transport_layer_2: String = transport_layer.clone();
    let app_layer = from_name_to_application_protocol(args.application_layer_filter.to_ascii_lowercase());

    if args.device_list {
        print_device_list();
        return;
    }

    if !is_valid_network_layer(network_layer.clone()) {
        eprint!("{}","\r\n\tERROR: Specified network layer filter must be equal to 'IPv4' or 'IPv6' (not case sensitive).\r\n\n".red().bold());
        return;
    }

    if !is_valid_transport_layer(transport_layer.clone()) {
        eprint!("{}","\r\n\tERROR: Specified transport layer filter must be equal to 'TCP' or 'UDP' (not case sensitive).\r\n\n".red().bold());
        return;
    }

    if app_layer.is_none() {
        eprint!("{}","\r\n\tERROR: Specified application layer protocol is unknown.\r\n\n".red().bold());
        return;
    }

    if lowest_port > highest_port {
        eprint!("{}", "\r\n\tERROR: Specified lowest port is greater than specified highest port.\r\n\n".red().bold());
        return;
    }

    if interval == 0 {
        eprint!("{}", "\r\n\tERROR: Specified time interval is null.\r\n\n".red().bold());
        return;
    }

    let found_device_option = retrieve_device(&mut adapter);

    if found_device_option.is_none() {
        eprint!("{}", "\r\n\tERROR: Specified network adapter does not exist. Use option '-d' to list all the available devices.\r\n\n".red());
        return;
    }

    let found_device = found_device_option.unwrap();
    let device_name = found_device.clone().name;

    //shared tuple containing:
    // - the map of the address:ports pairs with the relative info
    // - the total number of sniffed packets
    // - the number of filtered packets
    // - the map of the observed app protocols with the relative packet count
    let mutex_map1 = Arc::new(Mutex::new(InfoTraffic::new()));
    let mutex_map2 = mutex_map1.clone();

    //shared tuple containing the application status and the relative condition variable
    let status_pair1 = Arc::new((Mutex::new(Status::Running), Condvar::new()));
    let status_pair2 = status_pair1.clone();
    let status_pair3 = status_pair1.clone();

    println!("{}{}{}", "\r\n\n\tSniffing network adapter '".cyan().italic(), device_name.cyan().italic(), "'\r".cyan().italic());
    println!("{}{}{}", "\tThe folder '".cyan().italic(), output_folder.cyan().italic(),
              "' will be periodically updated\r".cyan().italic());
    println!("{}{}{}{}", "\r\n\tPress the key\r".cyan().bold(),  "\r\n\t\t- 'p' to pause\r".yellow().bold(),
             "\r\n\t\t- 's' to stop\r".red().bold(), "\r\n\tthe application\n\r".cyan().bold());
    print!("\t{}{}{}\r", "Updating reports every ".cyan().blink().italic(),
           interval.to_string().cyan().blink().italic(),
           " seconds...".cyan().blink().italic());
    io::stdout().flush().unwrap();

    // to kill the main thread even if a secondary thread panics
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // disable raw mode (brute force exit seems to not drop _raw of set_status_by_key() function)
        let _raw = RawScreen::disable_raw_mode();
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    // Thread 1: updates textual report
    let thread_write_report = thread::spawn(move || {
        sleep_and_write_report_loop(lowest_port, highest_port, interval, min_packets,
                                    device_name, network_layer,
                                    transport_layer, app_layer.unwrap(), output_folder,
                                    mutex_map2, status_pair3);
    });

    // Thread 2: parses packets
    thread::spawn(move || {
        parse_packets_loop(found_device, lowest_port, highest_port, network_layer_2,
                           transport_layer_2, app_layer.unwrap(), mutex_map1, status_pair1);
    });

    // Main thread: updates application status
    set_status_by_key(status_pair2, interval);

    // Wait for the final report update, to not kill the application while the report is being written
    thread_write_report.join().expect("Thread in charge of writing report panicked!\r\n");

}


/// Prints the list of available network adapters' names and addresses.
///
/// This function is called if the user specifies the ```-d``` command line option.
fn print_device_list() {
    println!("\r");
    for dev in Device::list().expect("Error retrieving device list\r\n") {
        match dev.desc {
            None => {
                print!("\r\tDevice: {}\r\n\t\tAddresses: ", dev.name.cyan());
            }
            Some(description) => {
                print!("\r\tDevice: {} ({})\r\n\t\tAddresses: ",  dev.name.cyan(), description.cyan());
            }
        }
        if dev.addresses.is_empty() {
            println!("\r");
        }
        for addr in dev.addresses {
            let address_string = addr.addr.to_string();
            print!("{}\r\n\t\t\t   ", address_string.cyan());
        }
        println!("\r");
    }
    println!("\r");
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
        *adapter = Device::lookup().expect("Error retrieving default network adapter\r\n").expect("Error: no suitable devices\r\n").name;
    }
    let dev_list = Device::list().expect("Unable to retrieve network adapters list\r\n");
    for device in dev_list {
        if device.name == *adapter {
            found_device = Some(device);
            break;
        }
    }
    found_device
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
fn set_status_by_key(status_pair: Arc<(Mutex<Status>, Condvar)>, interval: u64) {

    let _raw = RawScreen::into_raw_mode();
    let mut reader = input().read_sync();
    let cvar = &status_pair.1;
    loop {
        if let Some(event) = reader.next() { // Blocking call
            let mut status = status_pair.0.lock().expect("Error acquiring mutex\r\n");
            match event {
                InputEvent::Keyboard(KeyEvent::Char('p')) => {
                    if *status == Status::Running {
                        print!("                                                              \r");
                        io::stdout().flush().unwrap();
                        println!("\t{}", "Sniffnet paused...\r".yellow().bold());
                        print!("\t{}", "Press 'r' to resume\r".yellow().blink().bold());
                        io::stdout().flush().unwrap();
                        *status = Status::Pause;
                    }
                }
                InputEvent::Keyboard(KeyEvent::Char('r')) => {
                    if *status == Status::Pause {
                        print!("                                                              \r");
                        io::stdout().flush().unwrap();
                        println!("\t{}", "Sniffnet resumed\r".green().bold());
                        print!("\t{}{}{}\r", "Updating reports every ".cyan().blink().italic(),
                               interval.to_string().cyan().blink().italic(),
                               " seconds...".cyan().blink().italic());
                        io::stdout().flush().unwrap();
                        *status = Status::Running;
                        cvar.notify_all();
                    }
                }
                InputEvent::Keyboard(KeyEvent::Char('s')) => {
                    print!("                                                              \r");
                    io::stdout().flush().unwrap();
                    println!("\r\t{}", "Sniffnet stopped... waiting for the last reports update\n\n\r".red().bold());
                    *status = Status::Stop;
                    cvar.notify_all();
                    return;
                }
                _ => { /* Other events */ }
            }
        }
    }
}


/// Given a String representing an application layer protocol, this function returns an `Option<AppProtocol>` containing
/// the respective application protocol represented by a value of the `AppProtocol` enum.
/// Only the most common application layer protocols are considered; if a unknown protocol
/// is provided, this function returns `None`.
///
/// # Arguments
///
/// * `name` - A String representing an application layer protocol
///
/// # Examples
///
/// ```
/// let x = from_name_to_application_protocol("smtp".to_string());
/// //Simple Mail Transfer Protocol
/// assert_eq!(x, Option::Some(AppProtocol::SMTP));
///
/// let y = from_name_to_application_protocol("not a known app protocol".to_string());
/// //Unknown port-to-protocol mapping
/// assert_eq!(y, Option::None);
/// ```
fn from_name_to_application_protocol(name: String) -> Option<AppProtocol> {
    match name.as_str() {
        "ftp" => {Option::Some(AppProtocol::FTP)},
        "ssh" => {Option::Some(AppProtocol::SSH)},
        "telnet" => {Option::Some(AppProtocol::Telnet)},
        "smtp" => {Option::Some(AppProtocol::SMTP)},
        "tacacs" => {Option::Some(AppProtocol::TACACS)},
        "dns" => {Option::Some(AppProtocol::DNS)},
        "dhcp" => {Option::Some(AppProtocol::DHCP)},
        "tftp" => {Option::Some(AppProtocol::TFTP)},
        "http" => {Option::Some(AppProtocol::HTTP)},
        "pop" => {Option::Some(AppProtocol::POP)},
        "ntp" => {Option::Some(AppProtocol::NTP)},
        "netbios" => {Option::Some(AppProtocol::NetBIOS)},
        "imap" => {Option::Some(AppProtocol::IMAP)},
        "snmp" => {Option::Some(AppProtocol::SNMP)},
        "bgp" => {Option::Some(AppProtocol::BGP)},
        "ldap" => {Option::Some(AppProtocol::LDAP)},
        "https" => {Option::Some(AppProtocol::HTTPS)},
        "ldaps" => {Option::Some(AppProtocol::LDAPS)},
        "ftps" => {Option::Some(AppProtocol::FTPS)},
        "imaps" => {Option::Some(AppProtocol::IMAPS)},
        "pop3s" => {Option::Some(AppProtocol::POP3S)},
        "ssdp" => {Option::Some(AppProtocol::SSDP)},
        "xmpp" => {Option::Some(AppProtocol::XMPP)},
        "mdns" => {Option::Some(AppProtocol::mDNS)},
        "no filter" => {Option::Some(AppProtocol::Other)},
         _ => {None}
    }
}