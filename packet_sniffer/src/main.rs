mod address_port;
mod report_info;
mod args;
mod thread_parse_packets_functions;
mod thread_write_report_functions;

use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use pcap::{Device, Capture, Stat};
use crate::address_port::{AddressPort};
use crate::report_info::{AppProtocol, ReportInfo, TransProtocol};
use crate::args::Args;
use crate::thread_parse_packets_functions::parse_packets_loop;
use crate::thread_write_report_functions::sleep_and_write_report_loop;
use clap::Parser;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, Condvar};
use crossterm::{screen::RawScreen,  input::{input, InputEvent, KeyEvent}};

#[derive(PartialEq, Eq)]
pub enum Status {Running, Pause, Stop}

/// Entry point of application execution.
fn main() {
    let args = Args::parse();
    let adapter: String = args.adapter;
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

    if  !is_valid_network_layer(network_layer.clone()) {
        eprint!("\n\tERROR: Specified network layer filter must be equal to 'IPv4' or 'IPv6' (not case sensitive).\n\n");
        return;
    }

    if !is_valid_transport_layer(transport_layer.clone()) {
        eprint!("\n\tERROR: Specified transport layer filter must be equal to 'TCP' or 'UDP' (not case sensitive).\n\n");
        return;
    }

    if lowest_port > highest_port {
        eprint!("\n\tERROR: Specified lowest port is greater than specified highest port.\n\n");
        return;
    }

    if interval == 0 {
        eprint!("\n\tERROR: Specified time interval is null.\n\n");
        return;
    }

    let found_device_option = retrieve_device(adapter);

    if found_device_option.is_none() {
        eprint!("\n\tERROR: Specified network adapter does not exist. Use option '-d' to list all the available devices.\n\n");
        return;
    }

    let found_device = found_device_option.unwrap();

    let cap = Capture::from_device(found_device.clone())
        .expect("Capture initialization error\n")
        .promisc(true)
        .buffer_size(10_000_000)
        .open()
        .expect("Capture initialization error\n");

    let mutex_map1 = Arc::new(Mutex::new(HashMap::new()));
    let mutex_map2 = mutex_map1.clone();
    let status_pair1 = Arc::new((Mutex::new(Status::Running), Condvar::new()));
    let status_pair2 = status_pair1.clone();
    let status_pair3 = status_pair1.clone();

    println!("\n\tParsing packets...");
    println!("\tUpdating the file '{}' every {} seconds\n", output_file, interval);

    thread::spawn(move || {
        sleep_and_write_report_loop(lowest_port, highest_port, interval, min_packets,
                                    found_device.name, network_layer,
                                    transport_layer, output_file,
                                    mutex_map2, status_pair3);
    });
    //let (m_stat, cvar) = &*status_pair1;

    set_status_by_key(status_pair2);

    parse_packets_loop(cap, lowest_port, highest_port, network_layer_2,
                       transport_layer_2, mutex_map1, status_pair1);
}


/// Prints the list of available network adapters' names and addresses.
///
/// This function is called if the user specifies the ```-d``` command line option.
fn print_device_list() {
    println!();
    for dev in Device::list().expect("Error retrieving device list\n") {
        print!("\tDevice: {}\n\t\tAddresses: ", dev.name);
        if dev.addresses.len() == 0 {
            println!();
        }
        for addr in dev.addresses {
            print!("{:?}\n\t\t\t   ", addr.addr);
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
fn retrieve_device(adapter: String) -> Option<Device> {
    let mut found_device = None;
    if adapter.eq(&"default".to_string()) {
        found_device = Some(Device::lookup().expect("Error retrieving default network adapter\n"));
    } else {
        let dev_list = Device::list().expect("Unable to retrieve network adapters list\n");
        for device in dev_list {
            if device.name == adapter {
                found_device = Some(device);
                break;
            }
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

fn set_status_by_key(status_pair: Arc<(Mutex<Status>, Condvar)>) {
    thread::spawn(move || {
        let _raw = RawScreen::into_raw_mode();
        let mut reader = input().read_sync();
        let cvar = &status_pair.1;
        loop {
            if let Some(event) = reader.next() { // Blocking call
                let mut status = status_pair.0.lock().unwrap();
                match event {
                    InputEvent::Keyboard(KeyEvent::Char('p')) => {
                        if *status == Status::Running {
                            println!("Capture paused... Press 'r' to resume.");
                            *status = Status::Pause;
                        }
                    }
                    InputEvent::Keyboard(KeyEvent::Char('r')) => {
                        if *status == Status::Pause {
                            println!("Capture resumed");
                            *status = Status::Running;
                            cvar.notify_all();
                        }
                    }
                    InputEvent::Keyboard(KeyEvent::Esc) => {
                        *status = Status::Stop;
                        cvar.notify_all();
                        break;
                    }
                    _ => { /* Other events */ }
                }
            }
            thread::sleep(Duration::from_millis(50));
        }
    });

}