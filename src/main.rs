mod thread_parse_packets;
mod address_port_pair;
mod info_address_port_pair;
mod thread_write_report;
mod info_traffic;
mod style;
mod app;
mod gui_initial_page;
mod gui_run_page;
mod charts_data;

use pcap::{Device};
use crate::info_address_port_pair::{AppProtocol, TransProtocol};
use crate::thread_parse_packets::parse_packets_loop;
use crate::thread_write_report::sleep_and_write_report_loop;
use crate::thread_write_report::get_app_count_string;
use crate::gui_run_page::TrafficChart;
use std::{panic, process, thread};
use std::sync::{Arc, Mutex, Condvar};
use iced::{Application, button, pick_list, scrollable, Settings, window};
use crate::info_traffic::InfoTraffic;
use style::{Mode, FONT_SIZE_BODY, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE, icon_sun_moon};
use crate::charts_data::ChartsData;


pub struct Filters {
    ip: String,
    transport: TransProtocol,
    application: AppProtocol,
}


pub struct Sniffer {
    current_capture_id: Arc<Mutex<u16>>,
    info_traffic: Arc<Mutex<InfoTraffic>>,
    charts_data: Arc<Mutex<ChartsData>>,
    device: Arc<Mutex<Device>>,
    filters: Arc<Mutex<Filters>>,
    status_pair: Arc<(Mutex<Status>, Condvar)>,
    start: button::State,
    reset: button::State,
    mode: button::State,
    report: button::State,
    git: button::State,
    app: pick_list::State<AppProtocol>,
    scroll_adapters: scrollable::State,
    scroll_packets: scrollable::State,
    scroll_report: scrollable::State,
    style: Mode,
    waiting: String,
    traffic_chart: TrafficChart,
    chart_packets: bool,
    report_latest: bool,
}


/// This enum represents the sniffing process status.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Status {
    /// Sniffnet has just been launched/restarted and GUI is in the main screen.
    Init,
    /// The sniffing process is running: the application parses packets and periodically update the output report.
    Running,
    /// The sniffing process is killed.
    Stop,
}

pub fn main() -> iced::Result {
    let current_capture_id1 = Arc::new(Mutex::new(0));
    let current_capture_id2 = current_capture_id1.clone();

    //shared tuple containing:
    // - the map of the address:ports pairs with the relative info
    // - the total number of sniffed packets
    // - the number of filtered packets
    // - the map of the observed app protocols with the relative packet count
    let mutex_map1 = Arc::new(Mutex::new(InfoTraffic::new()));
    let mutex_map2 = mutex_map1.clone();

    let charts_data1 = Arc::new(Mutex::new(ChartsData::new()));
    let charts_data2 = charts_data1.clone();

    //shared tuple containing the application status and the relative condition variable
    let status_pair1 = Arc::new((Mutex::new(Status::Init), Condvar::new()));
    let status_pair2 = status_pair1.clone();

    let found_device1 = Arc::new(Mutex::new(Device::lookup().unwrap().unwrap()));
    let found_device2 = found_device1.clone();

    let filters1 = Arc::new(Mutex::new(Filters {
        ip: "no filter".to_string(),
        transport: TransProtocol::Other,
        application: AppProtocol::Other,
    }));
    let filters2 = filters1.clone();

    // to kill the main thread as soon as a secondary thread panics
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    thread::Builder::new().name("thread_write_report".to_string()).spawn(move || {
        sleep_and_write_report_loop(current_capture_id2, 0, 65535, 1,
                                    found_device2, filters2, "./sniffnet_report".to_string(),
                                    mutex_map2, status_pair2);
    }).unwrap();

    Sniffer::run(Settings {
        id: None,
        window: window::Settings {
            size: (1280, 800),
            position: Default::default(),
            min_size: Some((1185, 0)), //allow to reduce the most I can sustain
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
        },
        flags: Sniffer {
            current_capture_id: current_capture_id1,
            info_traffic: mutex_map1,
            charts_data: charts_data1,
            device: found_device1,
            filters: filters1,
            status_pair: status_pair1,
            start: button::State::new(),
            reset: button::State::new(),
            mode: button::State::new(),
            report: button::State::new(),
            git: button::State::new(),
            app: pick_list::State::new(),
            scroll_adapters: scrollable::State::new(),
            scroll_packets: scrollable::State::new(),
            scroll_report: scrollable::State::new(),
            style: Mode::Night,
            waiting: String::new(),
            traffic_chart: TrafficChart::new(charts_data2),
            chart_packets: true,
            report_latest: true
        },
        default_font: None,
        default_text_size: FONT_SIZE_BODY,
        text_multithreading: true,
        antialiasing: false,
        exit_on_close_request: true,
        try_opengles_first: false,
    })
}