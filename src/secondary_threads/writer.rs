use pcap::*;
use std::sync::mpsc::Receiver;

pub fn writer(cap: &Capture<Active>, rx: Receiver<Packet>) {
    // let mut cap = Capture::from_device(device)
    //     .unwrap()
    //     .immediate_mode(true)
    //     .open()
    //     .unwrap();

    let mut savefile = cap.savefile("test.pcap").unwrap();
    loop {
        let packet = rx.recv().unwrap();
        savefile.write(&packet);
    }
}
