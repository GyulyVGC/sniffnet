//! Module containing functions executed by the thread in charge of updating the output report
//! every ```interval``` seconds, with ```interval``` specified by the user through the
//! ```-i``` command line option.
//!
//! If the ```-i``` option is not specified, the report is updated every 5 seconds.

use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration};
use std::{fs, thread};
use chrono::{Local};
use std::io::{BufWriter, Seek, SeekFrom, Write};
use colored::Colorize;
use thousands::Separable;
use plotters::prelude::*;
use crate::{AppProtocol, InfoTraffic, Status, TransProtocol};

use std::time::{Instant};
use plotters::style::full_palette::{GREEN_800, GREY};


/// The calling thread enters in a loop in which it waits for ```interval``` seconds and then re-write
/// from scratch the output report file, with updated values.
///
/// # Arguments
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
/// * `device_name` - A String representing the name of th network adapter to be sniffed. Specified by the user through the
/// ```-a``` option.
///
/// * `network_layer` - A String representing the IP version to be filtered. Specified by the user through the
/// ```-n``` option.
///
/// * `transport_layer` - A TransProtocol representing the transport protocol to be filtered. Specified by the user through the
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
pub fn sleep_and_write_report_loop(lowest_port: u16, highest_port: u16, interval: u64, device_name: String,
                                   network_layer: String, transport_layer: TransProtocol, app_layer: AppProtocol,
                                   output_folder: String, info_traffic_mutex: Arc<Mutex<InfoTraffic>>,
                                   status_pair: Arc<(Mutex<Status>, Condvar)>) {

    if fs::create_dir(output_folder.clone()).is_err() {
        fs::remove_dir_all(output_folder.clone()).unwrap();
        fs::create_dir(output_folder.clone()).unwrap();
    }

    let path_graph = &*format!("{}/bandwidth.svg", output_folder);
    let path_report = format!("{}/report.txt", output_folder.clone());
    let path_statistics = format!("{}/statistics.txt", output_folder.clone());

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

    let mut output = BufWriter::new(File::create(path_report.clone()).expect("Error creating output file\n\r"));
    writeln!(output, "---------------------------------------------------------------------------------------------------------------------------------------------------------------------").expect("Error writing output file\n\r");
    writeln!(output, "|     Src IP address      | Src port |     Dst IP address      | Dst port | Layer 4 | Layer 7 |   Packets  |   Bytes    |  Initial timestamp  |   Final timestamp   |").expect("Error writing output file\n\r");
    writeln!(output, "---------------------------------------------------------------------------------------------------------------------------------------------------------------------").expect("Error writing output file\n\r");

    loop {
        // sleep interval seconds
        thread::sleep(Duration::from_secs(interval));

        let tot_seconds = (Local::now() - time_origin).num_seconds();

        if *status_pair.0.lock().expect("Error acquiring mutex\n\r") != Status::Pause {

            let mut info_traffic = info_traffic_mutex.lock().expect("Error acquiring mutex\n\r");

            let tot_sent_packets = info_traffic.tot_sent_packets;
            let tot_received_packets = info_traffic.tot_received_packets;
            let all_packets = info_traffic.all_packets;
            let tot_sent_bytes = info_traffic.tot_sent_bytes;
            let tot_received_bytes = info_traffic.tot_received_bytes;

            start = Instant::now();

            let mut output2 = BufWriter::new(File::create(path_statistics.clone()).expect("Error creating output file\n\r"));

            write_statistics(output2.get_mut().try_clone().expect("Error cloning file handler\n\r"),
                                     device_name.clone(), first_timestamp.clone(),
                                     lowest_port, highest_port, network_layer.clone(),
                                     transport_layer.clone(), app_layer,
                                     info_traffic.map.len(), all_packets,
                                     tot_received_packets+tot_sent_packets,
                                     info_traffic.app_protocols.clone());
            output2.flush().expect("Error writing output file\n\r");

            _time_header = start.elapsed().as_millis();


            let _num_written_tuples_interval = info_traffic.addresses_last_interval.len();

            for key in info_traffic.addresses_last_interval.iter() {
                let val = info_traffic.map.get(key).unwrap();
                let index = info_traffic.map.get_index_of(key).unwrap();
                let seek_pos = 166*3 + 206*index as u64;
                output.seek(SeekFrom::Start(seek_pos)).unwrap();
                writeln!(output, "{}{}", key, val).expect("Error writing output file\n\r");
            }
            info_traffic.addresses_last_interval = HashSet::new(); // empty set

            output.flush().expect("Error writing output file\n\r");

            _time_header_sort_print = start.elapsed().as_millis();


            drop(info_traffic);

            // graphs
            _start_drawing = Instant::now();

            // update bits traffic data
            sent_bits_graph.push((tot_seconds as u128, (-1*(tot_sent_bytes*8) as i128 + tot_sent_bits_prev)/interval as i128 ));
            if -1*(tot_sent_bytes*8) as i128 + tot_sent_bits_prev < min_sent_bits_second {
                min_sent_bits_second = -1*(tot_sent_bytes*8) as i128 + tot_sent_bits_prev;
            }
            tot_sent_bits_prev = (tot_sent_bytes * 8) as i128;
            received_bits_graph.push((tot_seconds as u128, (tot_received_bytes as i128 * 8 - tot_received_bits_prev)/interval as i128 ));
            if tot_received_bytes as i128 * 8 - tot_received_bits_prev  > max_received_bits_second {
                max_received_bits_second = tot_received_bytes as i128 * 8 - tot_received_bits_prev ;
            }
            tot_received_bits_prev = (tot_received_bytes * 8) as i128;

            // update packets traffic data
            sent_packets_graph.push((tot_seconds as u128, (-(tot_sent_packets as i128) + tot_sent_packets_prev)/interval as i128 ));
            if -(tot_sent_packets as i128) + tot_sent_packets_prev < min_sent_packets_second {
                min_sent_packets_second = -(tot_sent_packets as i128) + tot_sent_packets_prev;
            }
            tot_sent_packets_prev = tot_sent_packets as i128;
            received_packets_graph.push((tot_seconds as u128, (tot_received_packets as i128 - tot_received_packets_prev)/interval as i128 ));
            if tot_received_packets as i128 - tot_received_packets_prev > max_received_packets_second {
                max_received_packets_second = tot_received_packets as i128 - tot_received_packets_prev;
            }
            tot_received_packets_prev = tot_received_packets as i128;


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
                .build_cartesian_2d(0..tot_seconds as u128, min_sent_bits_second/interval as i128..max_received_bits_second/interval as i128)
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
                .build_cartesian_2d(0..tot_seconds as u128, min_sent_packets_second/interval as i128..max_received_packets_second/interval as i128)
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

            #[cfg(feature = "elapsed_time")]
            {
                println!("---------------------------------------------------------\r\n\
            \t\tTimings (written tuples = {})\r\n\
            \t\t\tPrint header: {} ms\r\n\
            \t\t\tPrint map: {} ms\r\n\
            \t\t\tTot time mutex held: {} ms\r\n\
            \t\t\tDraw graphical report: {} ms\r\n",
                         _num_written_tuples_interval, _time_header,
                         _time_header_sort_print-_time_header_sort,
                         _time_header_sort_print, _start_drawing.elapsed().as_millis());
            }
        }
        else {
            sent_bits_graph.push((tot_seconds as u128,0));
            received_bits_graph.push((tot_seconds as u128,0));
            sent_packets_graph.push((tot_seconds as u128,0));
            received_packets_graph.push((tot_seconds as u128,0));
        }

        if *status_pair.0.lock().expect("Error acquiring mutex\n\r") == Status::Stop {
            println!("{}{}{}\r", "\tThe final reports are available in the folder '".cyan().italic(),
                     output_folder.cyan().bold(), "'\n\n\r".cyan().italic());
            return;
        }
    }
}


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
        format!("\t[x] Considering only port number {}\n", lowest_port)
    }
    else if lowest_port != u16::MIN || highest_port != u16::MAX {
        format!("\t[x] Considering only port numbers from {} to {}\n", lowest_port, highest_port)
    }
    else {
        format!("\t[ ] Considering all port numbers (from {} to {})\n", lowest_port, highest_port)
    }
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
        "\t[x] Considering only IPv4 packets\n".to_string()
    }
    else if network_layer.cmp(&"ipv6".to_string()) == Equal {
        "\t[x] Considering only IPv6 packets\n".to_string()
    }
    else {
        "\t[ ] Considering both IPv4 and IPv6 packets\n".to_string()
    }
}


/// Given the transport layer textual filter, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `transport_layer` - A TransProtocol representing the transport protocol to be filtered. Specified by the user through the
/// ```-t``` option.
fn get_transport_layer_string(transport_layer: TransProtocol) -> String {
    if transport_layer.eq(&TransProtocol::TCP) {
        "\t[x] Considering only packets exchanged with TCP\n".to_string()
    }
    else if transport_layer.eq(&TransProtocol::UDP) {
        "\t[x] Considering only packets exchanged with UDP\n".to_string()
    }
    else {
        "\t[ ] Considering packets exchanged both with TCP and/or UDP\n".to_string()
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
        "\t[ ] Considering all application layer protocols\n".to_string()
    }
    else {
        format!("\t[x] Considering only {:?} packets\n", app_layer)
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
        format!("\tConsidered packets: {} {}\n",
                filtered.separate_with_underscores(), percentage_string)
    }
    else {
        format!("\tConsidered packets: {}\n",
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

        ret_val.push_str(&format!("\t-{}:{}{}{}{}\n",
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
/// * `network_layer` - A String representing the IP version to be filtered. Specified by the user through the
/// ```-n``` option.
///
/// * `transport_layer` - A TransProtocol representing the transport protocol to be filtered. Specified by the user through the
/// ```-t``` option.
///
/// * `app_layer` - An AppProtocol representing the application protocol to be filtered. Specified by the user through the
/// ```--app``` option.
///
/// * `num_pairs` - Total numbers of address:port pairs considered in the report.
///
/// * `num_sniffed_packets` - Total numbers of sniffed packets.
/// ```
fn write_statistics(mut output: File, device_name: String, first_timestamp: String,
                            lowest_port: u16, highest_port: u16,
                            network_layer: String, transport_layer: TransProtocol, app_layer: AppProtocol,
                            num_pairs: usize, num_sniffed_packets: u128, num_filtered_packets: u128,
                            app_count: HashMap<AppProtocol, u128>) {

    let adapter_string = format!("Packets are sniffed from adapter '{}'\n", device_name);
    let first_timestamp_string = format!("\tReport start time: {}\n", first_timestamp);
    let last_timestamp_string = format!("\tReport last update: {}\n", Local::now().format("%d/%m/%Y %H:%M:%S"));
    let ports_string = get_ports_string(lowest_port,highest_port);
    let network_layer_string = get_network_layer_string(network_layer);
    let transport_layer_string = get_transport_layer_string(transport_layer);
    let app_layer_string = get_app_layer_string(app_layer);
    let filtered_packets_string = get_filtered_packets_string(num_sniffed_packets, num_filtered_packets);

    writeln!(output, "{}", adapter_string).expect("Error writing output file\n");

    writeln!(output, "Report updates info").expect("Error writing output file\n");
    write!(output, "{}", first_timestamp_string).expect("Error writing output file\n");
    writeln!(output, "{}", last_timestamp_string).expect("Error writing output file\n");

    writeln!(output, "Filters").expect("Error writing output file\n");
    write!(output, "{}", network_layer_string).expect("Error writing output file\n");
    write!(output, "{}", transport_layer_string).expect("Error writing output file\n");
    write!(output, "{}", ports_string).expect("Error writing output file\n");
    writeln!(output, "{}", app_layer_string).expect("Error writing output file\n");

    writeln!(output, "Overall statistics").expect("Error writing output file\n");
    writeln!(output, "\tConsidered [address:port] pairs: {}", num_pairs.separate_with_underscores()).expect("Error writing output file\n");
    writeln!(output, "\tTotal packets: {}", num_sniffed_packets.separate_with_underscores()).expect("Error writing output file\n");
    writeln!(output, "{}", filtered_packets_string).expect("Error writing output file\n");

    if num_sniffed_packets > 0 {
        let app_count_string = get_app_count_string(app_count, num_sniffed_packets);
        writeln!(output, "Total packets divided by app layer protocol").expect("Error writing output file\n");
        writeln!(output, "{}", app_count_string).expect("Error writing output file\n");
    }

}