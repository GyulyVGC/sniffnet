mod thread_parse_packets_functions;
mod address_port_pair;
mod info_address_port_pair;
mod args;
mod thread_write_report_functions;
mod info_traffic;
mod style;
mod app;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;
use font_awesome;
use plotters_iced::{Chart, ChartWidget, DrawingBackend, ChartBuilder};
use iced::{ Svg, alignment, button, scrollable, executor, Alignment, Application, Button, Column, Command, Container, Element, Length, Row, Settings, Subscription, Text, Color, Radio, Scrollable, PickList, pick_list, Font, widget};
use pcap::{Capture, Device};
use crate::info_address_port_pair::{AppProtocol, TransProtocol};
use crate::thread_parse_packets_functions::parse_packets_loop;
use std::cmp::Ordering::Equal;
use crate::args::Args;
use crate::thread_write_report_functions::sleep_and_write_report_loop;
use crate::thread_write_report_functions::get_app_count_string;
use clap::Parser;
use std::{io, panic, process, thread};
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::sync::{Arc, Mutex, Condvar};
use crossterm::{screen::RawScreen,  input::{input, InputEvent, KeyEvent}};
use colored::Colorize;
use iced::canvas::LineDash;
use iced::futures::FutureExt;
use iced_style::pane_grid::Line;
use indexmap::IndexMap;
use crate::info_traffic::InfoTraffic;
use style::{Mode, FontSizeBody, FontSizeSubtitle, FontSizeTitle, icon_sun_moon};


pub struct Filters {
    ip: String,
    transport: TransProtocol,
    application: AppProtocol
}


struct Sniffer {
    info_traffic: Arc<Mutex<InfoTraffic>>,
    device: Arc<Mutex<Device>>,
    filters: Arc<Mutex<Filters>>,
    status_pair: Arc<(Mutex<Status>, Condvar)>,
    start: button::State,
    reset: button::State,
    mode: button::State,
    report: button::State,
    app: pick_list::State<AppProtocol>,
    scroll: scrollable::State,
    style: Mode
}


/// This enum represents the sniffing process status.
#[derive(PartialEq, Eq)]
pub enum Status {
    /// Sniffnet has just been launched/restarted
    Init,
    /// The sniffing process is running: the application parses packets and periodically update the output report.
    Running,
    /// The sniffing process is pause by the user and waiting to be later resumed.
    Pause,
    /// The sniffing process is killed.
    Stop
}

pub fn main() {

    //shared tuple containing:
    // - the map of the address:ports pairs with the relative info
    // - the total number of sniffed packets
    // - the number of filtered packets
    // - the map of the observed app protocols with the relative packet count
    let mutex_map1 = Arc::new(Mutex::new(InfoTraffic::new()));
    let mutex_map2= mutex_map1.clone();
    let mutex_map3= mutex_map1.clone();

    //shared tuple containing the application status and the relative condition variable
    let status_pair1 = Arc::new((Mutex::new(Status::Init), Condvar::new()));
    let status_pair2 =  status_pair1.clone();
    let status_pair3 =  status_pair1.clone();

    let found_device1 = Arc::new(Mutex::new(Device::lookup().unwrap().unwrap()));
    let found_device2 = found_device1.clone();
    let found_device3 = found_device1.clone();

    let filters1 = Arc::new(Mutex::new(Filters {
        ip: "no filter".to_string(),
        transport: TransProtocol::Other,
        application: AppProtocol::Other
    }));
    let filters2 = filters1.clone();
    let filters3 = filters1.clone();

    thread::spawn(move || {
        sleep_and_write_report_loop(0, 65535, 1,
                                    found_device2, filters2, "./sniffnet_report".to_string(),
                                    mutex_map2, status_pair2);
    });

    thread::spawn(move || {
        parse_packets_loop(found_device1, 0, 65535,
                           filters1,
                           mutex_map1, status_pair1);
    });

    Sniffer::run(Settings {
        id: None,
        window: Default::default(),
        flags: Sniffer {
            info_traffic: mutex_map3,
            device: found_device3,
            filters: filters3,
            status_pair: status_pair3,
            start: button::State::new(),
            reset: button::State::new(),
            mode: button::State::new(),
            report: button::State::new(),
            app: pick_list::State::new(),
            scroll: scrollable::State::new(),
            style: Mode::Night
        },
        default_font: Some(include_bytes!("../fonts/CourierPrimeSans.ttf")),
        default_text_size: FontSizeBody,
        text_multithreading: false,
        antialiasing: false,

        exit_on_close_request: true,
        try_opengles_first: false
    });

}