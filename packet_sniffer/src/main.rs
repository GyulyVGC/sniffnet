use etherparse::PacketHeaders;
use pcap::{Device, Capture};

fn main() {

    let device = Device::lookup().unwrap();
    println!("{:?}", device);

    let mut cap = Capture::from_device(device).unwrap()
        .promisc(true)
        .snaplen(5000)
        .open().unwrap();

    while let Ok(packet) = cap.next() {
        match PacketHeaders::from_ethernet_slice(&packet) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {
                println!("----------------------------------");
                println!("link: {:?}", value.link);
                println!("ip: {:?}", value.ip);
                println!("transport: {:?}", value.transport);
                println!("----------------------------------");
            }
        }
    }
}