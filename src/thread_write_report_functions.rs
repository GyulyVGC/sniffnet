//! Module containing functions executed by the thread in charge of updating the output report
//! every ```interval``` seconds, with ```interval``` specified by the user through the
//! ```-i``` command line option.
//!
//! If the ```-i``` option is not specified, the report is updated every 5 seconds.

use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration};
use std::{fs, thread};
use chrono::{Local};
use std::io::{BufWriter, Write};
use colored::Colorize;
use thousands::Separable;
use plotters::prelude::*;
use crate::{address_port_pair::AddressPortPair, AppProtocol, info_address_port_pair::InfoAddressPortPair, InfoTraffic, Status};

use std::time::{Instant};
use plotters::style::full_palette::{GREEN_800, GREY};


/// The calling thread enters in a loop in which it waits for ```interval``` seconds and then re-write
/// from scratch the output report file, with updated values.
///
/// # Arguments
///
/// * `verbose` - Flag: if true textual report is verbose.
/// Specified by the user through the ```-v``` option.
///
/// * `lowest_port` - The lowest port number to be considered in the report. Specified by the user
/// through the ```-l``` option.
///
/// * `highest_port` - The highest port number to be considered in the report. Specified by the user
/// through the ```-h``` option.
///
/// * `interval` - Frequency of report updates (value in seconds). Specified by the user through the
/// ```-i``` option.
///
/// * `min_packets` - Minimum number of packets for an address:port pair to be considered in the report.
/// Specified by the user through the ```-m``` option.
///
/// * `device_name` - A String representing the name of th network adapter to be sniffed. Specified by the user through the
/// ```-a``` option.
///
/// * `network_layer` - A String representing the IP version to be filtered. Specified by the user through the
/// ```-n``` option.
///
/// * `transport_layer` - A String representing the transport protocol to be filtered. Specified by the user through the
/// ```-t``` option.
///
/// * `app_layer` - An AppProtocol representing the application protocol to be filtered. Specified by the user through the
/// ```--app``` option.
///
/// * `output_folder` - A String representing the folder to contain the reports. Specified by the user through the
/// ```-o``` option.
///
/// * `info_traffic_mutex` - Struct with all the relevant info on the network traffic analyzed.
///
/// * `status_pair` - Shared variable to check the application current status.
pub fn sleep_and_write_report_loop(verbose: bool, lowest_port: u16, highest_port: u16, interval: u64, min_packets: u128,
                                   device_name: String, network_layer: String, transport_layer: String, app_layer: AppProtocol,
                                   output_folder: String, info_traffic_mutex: Arc<Mutex<InfoTraffic>>,
                                   status_pair: Arc<(Mutex<Status>, Condvar)>) {

    if fs::create_dir(output_folder.clone()).is_err() {
        fs::remove_dir_all(output_folder.clone()).unwrap();
        fs::create_dir(output_folder.clone()).unwrap();
    }

    let path_graph = &*format!("{}/bandwidth.svg", output_folder);

    let mut tot_intervals: u128 = 0;
    let time_origin = Local::now();
    let first_timestamp = time_origin.format("%d/%m/%Y %H:%M:%S").to_string();


    let mut start;
    let mut _time_header = 0;
    let mut _time_header_sort = 0;
    let mut _time_header_sort_print = 0;
    let mut _start_drawing;

    let mut sent_bits_graph: Vec<(u128, i128)> = vec![(0, 0)];
    let mut tot_sent_bits_prev: i128 = 0;
    let mut min_sent_bits_second: i128 = 0;

    let mut received_bits_graph: Vec<(u128, i128)> = vec![(0, 0)];
    let mut tot_received_bits_prev: i128 = 0;
    let mut max_received_bits_second: i128 = 0;

    let mut sent_packets_graph: Vec<(u128, i128)> = vec![(0, 0)];
    let mut tot_sent_packets_prev: i128 = 0;
    let mut min_sent_packets_second: i128 = 0;

    let mut received_packets_graph: Vec<(u128, i128)> = vec![(0, 0)];
    let mut tot_received_packets_prev: i128 = 0;
    let mut max_received_packets_second: i128 = 0;

    let text_path = format!("{}/report.txt", output_folder.clone());

    loop {
        // sleep interval seconds
        thread::sleep(Duration::from_secs(interval));

        tot_intervals += 1;

        let info_traffic = info_traffic_mutex.lock().expect("Error acquiring mutex\n\r");

        let tot_sent_packets = info_traffic.tot_sent_packets;
        let tot_received_packets = info_traffic.tot_received_packets;
        let all_packets = info_traffic.all_packets;
        let tot_sent_bytes = info_traffic.tot_sent_bytes;
        let tot_received_bytes = info_traffic.tot_received_bytes;

        start = Instant::now();

        if *status_pair.0.lock().expect("Error acquiring mutex\n\r") != Status::Pause { // write textual report

            let mut output = BufWriter::new(File::create(text_path.clone()).expect("Error creating output file\n\r"));

            write_report_file_header(output.get_mut().try_clone().expect("Error cloning file handler\n\r"),
                                     device_name.clone(), first_timestamp.clone(),
                                     lowest_port, highest_port, min_packets,
                                     network_layer.clone(), transport_layer.clone(), app_layer,
                                     info_traffic.map.len(), all_packets,
                                     tot_received_packets+tot_sent_packets, info_traffic.app_protocols.clone());

            if !verbose {
                write!(output,"IP_src, port_src, IP_dest, port_dest, layer4, layer7, packets, bytes, first_timestamp, last_timestamp\n\n").expect("Error writing output file\n\r");
            }

            _time_header = start.elapsed().as_millis();

            let mut sorted_vec: Vec<(&AddressPortPair, &InfoAddressPortPair)> = info_traffic.map.iter().collect();
            sorted_vec.sort_by(|&(_, a), &(_, b)|
                b.transmitted_packets.cmp(&a.transmitted_packets));

            _time_header_sort = start.elapsed().as_millis();

            for (key, val) in sorted_vec.iter() {
                if val.transmitted_packets >= min_packets {
                    if !verbose { // concise
                        write!(output, "{}, {}, {}, {}, {:?}, {:?}, {}, {}, {}, {}\n",
                            key.address1, key.port1, key.address2, key.port2,
                               val.trans_protocol, val.app_protocol,
                            val.transmitted_packets, val.transmitted_bytes,
                            val.initial_timestamp, val.final_timestamp).expect("Error writing output file\n\r");
                    }
                    else { // verbose
                        write!(output, "{}\n{}\n\n", key, val).expect("Error writing output file\n\r");
                    }
                }
            }

            output.flush().expect("Error writing output file\n\r");

            _time_header_sort_print = start.elapsed().as_millis();

        }

        drop(info_traffic);

        // graphs
        _start_drawing = Instant::now();

        // update bits traffic data
        sent_bits_graph.push((interval as u128 * tot_intervals,(-1*(tot_sent_bytes*8) as i128 + tot_sent_bits_prev)/interval as i128 ));
        if -1*(tot_sent_bytes*8) as i128 + tot_sent_bits_prev < min_sent_bits_second {
            min_sent_bits_second = -1*(tot_sent_bytes*8) as i128 + tot_sent_bits_prev;
        }
        tot_sent_bits_prev = (tot_sent_bytes * 8) as i128;
        received_bits_graph.push((interval as u128 * tot_intervals, (tot_received_bytes as i128 * 8 - tot_received_bits_prev)/interval as i128 ));
        if tot_received_bytes as i128 * 8 - tot_received_bits_prev  > max_received_bits_second {
            max_received_bits_second = tot_received_bytes as i128 * 8 - tot_received_bits_prev ;
        }
        tot_received_bits_prev = (tot_received_bytes * 8) as i128;

        // update packets traffic data
        sent_packets_graph.push((interval as u128 * tot_intervals, (-(tot_sent_packets as i128) + tot_sent_packets_prev)/interval as i128 ));
        if -(tot_sent_packets as i128) + tot_sent_packets_prev < min_sent_packets_second {
            min_sent_packets_second = -(tot_sent_packets as i128) + tot_sent_packets_prev;
        }
        tot_sent_packets_prev = tot_sent_packets as i128;
        received_packets_graph.push((interval as u128 * tot_intervals, (tot_received_packets as i128 - tot_received_packets_prev)/interval as i128 ));
        if tot_received_packets as i128 - tot_received_packets_prev > max_received_packets_second {
            max_received_packets_second = tot_received_packets as i128 - tot_received_packets_prev;
        }
        tot_received_packets_prev = tot_received_packets as i128;


        if *status_pair.0.lock().expect("Error acquiring mutex\n\r") != Status::Pause { // update graph file

            // declare drawing area
            let root_area = SVGBackend::new(path_graph, (1280, 720)).into_drawing_area();
            root_area.fill(&GREY).expect("Error drawing graph");
            let (graphs_area, _) = root_area.split_horizontally(1255);
            let (bits_area, packets_area) = graphs_area.split_vertically(360);
            let (_, footer) = root_area.split_vertically(700);
            footer.titled(
                &*format!("Charts are updated every {} seconds", interval),
                ("helvetica", 16).into_font().color(&BLACK.mix(0.5)),
            ).expect("Error drawing graph");


            // bits graph

            let mut chart_bits = ChartBuilder::on(&bits_area)
                .set_label_area_size(LabelAreaPosition::Left, 60)
                .set_label_area_size(LabelAreaPosition::Bottom, 50)
                .caption("Bit traffic per second", ("helvetica", 30))
                .build_cartesian_2d(0..interval as u128 * tot_intervals, min_sent_bits_second/interval as i128..max_received_bits_second/interval as i128)
                .expect("Error drawing graph");
            chart_bits.configure_mesh()
                .y_desc("bit/s")
                .label_style(("helvetica", 16))
                .axis_desc_style(("helvetica", 16))
                .x_label_formatter(&|seconds| {
                    (time_origin+chrono::Duration::from_std(Duration::from_secs(*seconds as u64)).unwrap())
                        .format("%H:%M:%S").to_string()
                })
                .y_label_formatter(&|bits| {
                    match bits {
                        0..=999 | -999..=-1 => { format!("{}",bits) },
                        1000..=999_999 | -999_999..=-1000 => { format!("{:.1} {}",*bits as f64/1_000_f64, "k") },
                        1_000_000..=999_999_999 | -999_999_999..=-1_000_000 => { format!("{:.1} {}",*bits as f64/1_000_000_f64, "M") },
                        _ => { format!("{:.1} {}",*bits as f64/1_000_000_000_f64, "G") }
                    }
                })
                .draw().unwrap();
            chart_bits.draw_series(
                AreaSeries::new(received_bits_graph.iter().copied(), 0, GREEN_800.mix(0.2))
                    .border_style(&GREEN_800))
                .expect("Error drawing graph")
                .label("Incoming bits")
                .legend(|(x,y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], GREEN_800.filled()));
            chart_bits.draw_series(
                AreaSeries::new(sent_bits_graph.iter().copied(), 0, BLUE.mix(0.2))
                    .border_style(&BLUE))
                .expect("Error drawing graph")
                .label("Outgoing bits")
                .legend(|(x,y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], BLUE.filled()));
            chart_bits.configure_series_labels().position(SeriesLabelPosition::UpperRight).margin(5)
                .border_style(BLACK).label_font(("helvetica", 16)).draw().expect("Error drawing graph");



            // packets graph

            let mut chart_packets = ChartBuilder::on(&packets_area)
                .set_label_area_size(LabelAreaPosition::Left, 60)
                .set_label_area_size(LabelAreaPosition::Bottom, 50)
                .caption("Packet traffic per second", ("helvetica", 30))
                .build_cartesian_2d(0..interval as u128*tot_intervals, min_sent_packets_second/interval as i128..max_received_packets_second/interval as i128)
                .expect("Error drawing graph");
            chart_packets.configure_mesh()
                .y_desc("packet/s")
                .label_style(("helvetica", 16))
                .axis_desc_style(("helvetica", 16))
                .x_label_formatter(&|seconds| {
                    (time_origin+chrono::Duration::from_std(Duration::from_secs(*seconds as u64)).unwrap())
                        .format("%H:%M:%S").to_string()
                })
                .draw().unwrap();
            chart_packets.draw_series(
                AreaSeries::new(received_packets_graph.iter().copied(), 0, GREEN_800.mix(0.2))
                    .border_style(&GREEN_800))
                .expect("Error drawing graph")
                .label("Incoming packets")
                .legend(|(x,y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], GREEN_800.filled()));
            chart_packets.draw_series(
                AreaSeries::new(sent_packets_graph.iter().copied(), 0, BLUE.mix(0.2))
                    .border_style(&BLUE))
                .expect("Error drawing graph")
                .label("Outgoing packets")
                .legend(|(x,y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], BLUE.filled()));
            chart_packets.configure_series_labels().position(SeriesLabelPosition::UpperRight).margin(5)
                .border_style(BLACK).label_font(("helvetica", 16)).draw().expect("Error drawing graph");

            // draw graphs on file
            root_area.present().expect("Error drawing graph");

        }


        #[cfg(feature = "elapsed_time")]
        {
            println!("---------------------------------------------------------\r\n\
            \t\tTimings:\r\n\
            \t\t\tPrint header: {} ms\r\n\
            \t\t\tSort map: {} ms\r\n\
            \t\t\tPrint map: {} ms\r\n\
            \t\t\tTot time mutex held: {} ms\r\n\
            \t\t\tDraw graphical report: {} ms\r\n",
                   _time_header, _time_header_sort-_time_header,
                   _time_header_sort_print-_time_header_sort,
                   _time_header_sort_print, _start_drawing.elapsed().as_millis());
        }

        if *status_pair.0.lock().expect("Error acquiring mutex\n\r") == Status::Stop {
            println!("{}{}{}\r", "\tThe final reports are available in the folder '".cyan().italic(),
                     output_folder.cyan().bold(), "'\n\n\r".cyan().italic());
            return;
        }
    }
}


// #[cfg(test)]
// mod print_report_entry_benchmark {
//     extern crate test;
//
//     use std::collections::HashSet;
//     use std::fs::File;
//     use std::io::BufWriter;
//     use std::io::Write;
//     use test::Bencher;
//     use chrono::{DateTime, Local};
//     use crate::address_port_pair::{AddressPortPair, TrafficType};
//     use crate::info_address_port_pair::InfoAddressPortPair;
//     use crate::{AppProtocol, TransProtocol};
//
//     #[bench]
//     fn bench_print_verbose(b: &mut Bencher) {
//         let mut bench_vec: Vec<(&AddressPortPair, &InfoAddressPortPair)> = vec![];
//         let key = AddressPortPair::new
//             ("255.255.255.255".to_string(),
//              443,
//             "245.78.32.123".to_string(),
//             40900,
//             TrafficType::Outgoing
//             );
//         let now_ugly: DateTime<Local> = Local::now();
//         let now = now_ugly.format("%d/%m/%Y %H:%M:%S").to_string();
//         let val = InfoAddressPortPair {
//             transmitted_bytes: 5002222896,
//             transmitted_packets: 89394742,
//             initial_timestamp: now.clone(),
//             final_timestamp: now.clone(),
//             trans_protocols: HashSet::from([TransProtocol::TCP]),
//             app_protocol: AppProtocol::HTTPS
//         };
//         for _ in 0..3500 {
//             bench_vec.push((&key, &val));
//         }
//         let mut bench_output = BufWriter::new(File::create("bench.txt").expect("Error creating output file\n\r"));
//         b.iter(|| {
//             for (key, val) in bench_vec.iter() {
//                 write!(bench_output, "{}\n{}\n\n", key, val).expect("Error writing output file\n\r")
//             }
//             bench_output.flush().unwrap();
//         });
//     }
//
//     #[bench]
//     fn bench_print_concise(b: &mut Bencher) {
//         let mut bench_vec: Vec<(&AddressPortPair, &InfoAddressPortPair)> = vec![];
//         let key = AddressPortPair::new
//             ("255.255.255.255".to_string(),
//              443,
//              "245.78.32.123".to_string(),
//              40900,
//              TrafficType::Outgoing
//             );
//         let now_ugly: DateTime<Local> = Local::now();
//         let now = now_ugly.format("%d/%m/%Y %H:%M:%S").to_string();
//         let val = InfoAddressPortPair {
//             transmitted_bytes: 5002222896,
//             transmitted_packets: 89394742,
//             initial_timestamp: now.clone(),
//             final_timestamp: now.clone(),
//             trans_protocols: HashSet::from([TransProtocol::TCP]),
//             app_protocol: AppProtocol::HTTPS
//         };
//         for _ in 0..3500 {
//             bench_vec.push((&key, &val));
//         }
//         let mut bench_output = BufWriter::new(File::create("bench.txt").expect("Error creating output file\n\r"));
//         b.iter(|| {
//             for (key, val) in bench_vec.iter() {
//                 write!(bench_output, "{}, {}, {}, {}, {}, {}, {:?}, {:?}, {}, {}\n",
//                        key.address1, key.port1, key.address2, key.port2,
//                        val.transmitted_packets, val.transmitted_bytes,
//                        val.trans_protocols, val.app_protocol,
//                        val.initial_timestamp, val.final_timestamp).expect("Error writing output file\n\r");
//             }
//             bench_output.flush().unwrap();
//         });
//     }
// }



/// Given the lowest and highest port numbers, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `lowest_port` - The lowest port number to be considered in the report. Specified by the user
/// through the ```-l``` option.
///
/// * `highest_port` - The highest port number to be considered in the report. Specified by the user
/// through the ```-h``` option.
fn get_ports_string(lowest_port: u16, highest_port: u16) -> String {
    if lowest_port == highest_port {
        format!("<><>\t\t\t[x] Considering only port number {}\n", lowest_port)
    }
    else if lowest_port != u16::MIN || highest_port != u16::MAX {
        format!("<><>\t\t\t[x] Considering only port numbers from {} to {}\n", lowest_port, highest_port)
    }
    else {
        format!("<><>\t\t\t[ ] Considering all port numbers (from {} to {})\n", lowest_port, highest_port)
    }
}


/// Given the minimum packets number, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `min_packets` - Minimum number of packets for an address:port pair to be considered in the report.
/// Specified by the user through the ```-m``` option.
fn get_min_packets_string(min_packets: u128) -> String {
    format!("<><>\t\tShowing only [address:port] pairs featured by more than {} packets\n", min_packets)
}


/// Given the network layer textual filter, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `network_layer` - A String representing the IP version to be filtered. Specified by the user through the
/// ```-n``` option.
fn get_network_layer_string (network_layer: String) -> String {
    if network_layer.cmp(&"ipv4".to_string()) == Equal {
        "<><>\t\t\t[x] Considering only IPv4 packets\n".to_string()
    }
    else if network_layer.cmp(&"ipv6".to_string()) == Equal {
        "<><>\t\t\t[x] Considering only IPv6 packets\n".to_string()
    }
    else {
        "<><>\t\t\t[ ] Considering both IPv4 and IPv6 packets\n".to_string()
    }
}


/// Given the transport layer textual filter, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `transport_layer` - A String representing the transport protocol to be filtered. Specified by the user through the
/// ```-t``` option.
fn get_transport_layer_string(transport_layer: String) -> String {
    if transport_layer.cmp(&"tcp".to_string()) == Equal {
        "<><>\t\t\t[x] Considering only packets exchanged with TCP\n".to_string()
    }
    else if transport_layer.cmp(&"udp".to_string()) == Equal {
        "<><>\t\t\t[x] Considering only packets exchanged with UDP\n".to_string()
    }
    else {
        "<><>\t\t\t[ ] Considering packets exchanged both with TCP and/or UDP\n".to_string()
    }
}


/// Given the application layer filter, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `app_layer` - A String representing the application layer protocol to be filtered. Specified by the user through the
/// ```--app``` option.
fn get_app_layer_string(app_layer: AppProtocol) -> String {
    if app_layer.eq(&AppProtocol::Other) {
        "<><>\t\t\t[ ] Considering all application layer protocols\n".to_string()
    }
    else {
        format!("<><>\t\t\t[x] Considering only {:?} packets\n", app_layer)
    }
}


/// Given the numbers of sniffed packets and filtered packets, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `sniffed` - Number of sniffed packets.
///
/// * `filtered` - Number of filtered packets
fn get_filtered_packets_string(sniffed: u128, filtered: u128) -> String {
    if sniffed != 0 && filtered != 0 {
        let percentage_string =
            if format!("{:.2}", 100.0*filtered as f32/sniffed as f32).eq("0.00") {
                "(<0.01%)".to_string()
            }
            else {
                format!("({:.2}%)", 100.0*filtered as f32/sniffed as f32)
            };
        format!("<><>\t\t\tConsidered packets: {} {}\n",
                filtered.separate_with_underscores(), percentage_string)
    }
    else {
        format!("<><>\t\t\tConsidered packets: {}\n",
                filtered.separate_with_underscores())
    }
}


/// Given the map of app layer protocols with the relative sniffed packets count,
/// the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `app_count` - Map of app layer protocols with the relative sniffed packets count
///
/// * `tot_packets` - Total number of sniffed packets
fn get_app_count_string(app_count: HashMap<AppProtocol, u128>, tot_packets: u128) -> String {

    let mut ret_val = "".to_string();

    let mut sorted_app_count: Vec<(&AppProtocol, &u128)> = app_count.iter().collect();
    sorted_app_count.sort_by(|&(p1, a), &(p2, b)| {
        if p1.eq(&AppProtocol::Other) {
            Ordering::Greater
        }
        else if p2.eq(&AppProtocol::Other) {
            Ordering::Less
        }
        else {
            b.cmp(a)
        }
    });

    //compute the length of the longest packet count string, used to align text
    let mut longest_num;
    longest_num = sorted_app_count.get(0).unwrap().1.separate_with_underscores().len();
    match app_count.get(&AppProtocol::Other) {
        None => {}
        Some(x) => {
            if x.separate_with_underscores().len() > longest_num {
                longest_num = x.separate_with_underscores().len();
            }
        }
    }

    for entry in sorted_app_count {

        let app_proto_string = format!("{:?}", entry.0);

        let num_string = entry.1.separate_with_underscores().to_string();

        let percentage_string =
            if format!("{:.2}", 100.0*(*entry.1) as f32/tot_packets as f32).eq("0.00") {
                "(<0.01%)".to_string()
            }
            else {
                format!("({:.2}%)", 100.0*(*entry.1) as f32/tot_packets as f32)
            };

        //to align digits
        let spaces_string_1 = " ".to_string()
            .repeat(9+longest_num-num_string.len()-app_proto_string.len());
        let spaces_string_2 = " ".to_string()
            .repeat(11-percentage_string.len());

        ret_val.push_str(&format!("<><>\t\t\t-{}:{}{}{}{}\n",
                                  app_proto_string,
                                  spaces_string_1,
                                  num_string,
                                  spaces_string_2,
                                  percentage_string));

    }

    ret_val
}


/// Writes the output report file header, which contains useful info about the sniffing process.
///
/// # Arguments
///
/// * `output` - A String representing the output report file name. Specified by the user through the
/// ```-o``` option.
///
/// * `device_name` - A String representing the name of th network adapter to be sniffed. Specified by the user through the
/// ```-a``` option.
///
/// * `first_timestamp` - A not formatted String representing the initial timestamp of the sniffing process.
///
/// * `lowest_port` - The lowest port number to be considered in the report. Specified by the user
/// through the ```-l``` option.
///
/// * `highest_port` - The highest port number to be considered in the report. Specified by the user
/// through the ```-h``` option.
///
/// * `min_packets` - Minimum number of packets for an address:port pair to be considered in the report.
/// Specified by the user through the ```-m``` option.
///
/// * `network_layer` - A String representing the IP version to be filtered. Specified by the user through the
/// ```-n``` option.
///
/// * `transport_layer` - A String representing the transport protocol to be filtered. Specified by the user through the
/// ```-t``` option.
///
/// * `app_layer` - An AppProtocol representing the application protocol to be filtered. Specified by the user through the
/// ```--app``` option.
///
/// * `num_pairs` - Total numbers of address:port pairs considered in the report.
///
/// * `num_sniffed_packets` - Total numbers of sniffed packets.
/// # Examples
/// An example of output report file header generated by this function is reported below.
///
/// ```<><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
/// <><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
/// <><>
/// <><>        Packets are sniffed from adapter 'en0'
/// <><>
/// <><>        Report updates info
/// <><>            Report start time: 15/08/2022 15:39:07
/// <><>            Report last update: 15/08/2022 15:39:52
/// <><>            Report update frequency: every 5 seconds
/// <><>            Number of times report was updated: 9
/// <><>
/// <><>        Filters
/// <><>            Considering only address:port pairs featured by more than 500 packets
/// <><>            Considering both IPv4 and IPv6 packets
/// <><>            Considering only packets exchanged with TCP
/// <><>            Considering only port number 443
/// <><>
/// <><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
/// <><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
/// ```
fn write_report_file_header(mut output: File, device_name: String, first_timestamp: String,
                            lowest_port: u16, highest_port: u16,
                            min_packets: u128, network_layer: String, transport_layer: String, app_layer: AppProtocol,
                            num_pairs: usize, num_sniffed_packets: u128, num_filtered_packets: u128,
                            app_count: HashMap<AppProtocol, u128>) {

    let cornice_string = "<><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>\n".to_string();
    let adapter_string = format!("<><>\t\tPackets are sniffed from adapter '{}'\n", device_name);
    let first_timestamp_string = format!("<><>\t\t\tReport start time: {}\n", first_timestamp);
    let last_timestamp_string = format!("<><>\t\t\tReport last update: {}\n", Local::now().format("%d/%m/%Y %H:%M:%S"));
    let ports_string = get_ports_string(lowest_port,highest_port);
    let network_layer_string = get_network_layer_string(network_layer);
    let transport_layer_string = get_transport_layer_string(transport_layer);
    let app_layer_string = get_app_layer_string(app_layer);
    let filtered_packets_string = get_filtered_packets_string(num_sniffed_packets, num_filtered_packets);

    write!(output, "{}", cornice_string).expect("Error writing output file\n");
    writeln!(output, "{}<><>", cornice_string).expect("Error writing output file\n");
    writeln!(output, "{}<><>", adapter_string).expect("Error writing output file\n");

    writeln!(output, "<><>\t\tReport updates info").expect("Error writing output file\n");
    write!(output, "{}", first_timestamp_string).expect("Error writing output file\n");
    writeln!(output, "{}<><>", last_timestamp_string).expect("Error writing output file\n");

    writeln!(output, "<><>\t\tFilters").expect("Error writing output file\n");
    write!(output, "{}", network_layer_string).expect("Error writing output file\n");
    write!(output, "{}", transport_layer_string).expect("Error writing output file\n");
    write!(output, "{}", ports_string).expect("Error writing output file\n");
    writeln!(output, "{}<><>", app_layer_string).expect("Error writing output file\n");

    writeln!(output, "<><>\t\tOverall statistics").expect("Error writing output file\n");
    writeln!(output, "<><>\t\t\tConsidered [address:port] pairs: {}", num_pairs.separate_with_underscores()).expect("Error writing output file\n");
    writeln!(output, "<><>\t\t\tTotal packets: {}", num_sniffed_packets.separate_with_underscores()).expect("Error writing output file\n");
    writeln!(output, "{}<><>", filtered_packets_string).expect("Error writing output file\n");

    if num_sniffed_packets > 0 {
        let app_count_string = get_app_count_string(app_count, num_sniffed_packets);
        writeln!(output, "<><>\t\tTotal packets divided by app layer protocol").expect("Error writing output file\n");
        writeln!(output, "{}<><>", app_count_string).expect("Error writing output file\n");
    }

    if min_packets > 1 {
        let min_packets_string = get_min_packets_string(min_packets);
        writeln!(output, "{}<><>", min_packets_string).expect("Error writing output file\n");
    }
    write!(output,"{}", cornice_string).expect("Error writing output file\n");
    write!(output,"{}\n\n\n", cornice_string).expect("Error writing output file\n");
}