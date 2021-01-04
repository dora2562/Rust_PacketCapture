extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use pnet::datalink::Channel::Ethernet;

fn handle_ethernet_frame(interface: &NetworkInterface, ethernet: &EthernetPacket) {
    println!(
        "{}: {} > {}",
        interface.name,
        ethernet.get_source(),
        ethernet.get_destination(),
    );
}

fn main() {
    let iface_name = "eth0";
    let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap_or_else(|| panic!("No such network interface: {}", iface_name));

    println!("{:?}", interface);

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("unhandled channel type: {}"),
        Err(e) => panic!("unable to create channel: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                handle_ethernet_frame(&interface, &EthernetPacket::new(packet).unwrap())
            }
            Err(_) => {
                continue;
            }
        }
    }
}
