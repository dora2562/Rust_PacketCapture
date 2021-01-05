extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;

use pnet::packet::Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct PacketWithInterface {
    interface: NetworkInterface,
    packet: Vec<u8>,
}

fn get_packet_id(interface: &NetworkInterface, ethernet: &EthernetPacket) -> u16 {
    let ip = Ipv4Packet::new(ethernet.payload()).unwrap();
    match ip.get_next_level_protocol(){
        IpNextHeaderProtocols::Tcp => {
            return ip.get_identification()
        }
        IpNextHeaderProtocols::Udp => {
            return ip.get_identification()
        }
        IpNextHeaderProtocols::Icmp => {
            return ip.get_identification()
        }
        _ => (),
    }
    return 0;
}

fn handle_ethernet_frame(interface: &NetworkInterface, ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            println!(
                "{}: {} -> {}", interface.name, ethernet.get_source(), ethernet.get_destination());
            let ip = Ipv4Packet::new(ethernet.payload()).unwrap();
            handle_ip_packet(&interface, &ip)
        }
        _ => (),
    }
}

fn handle_ip_packet(interface: &NetworkInterface, ip: &Ipv4Packet) {
    print!("    {}: ", ip.get_next_level_protocol());
    match ip.get_next_level_protocol(){
        IpNextHeaderProtocols::Tcp => {
            let tcp = TcpPacket::new(ip.payload()).unwrap();
            println!("{}:{} -> {}:{}", ip.get_source(), tcp.get_source(), ip.get_destination(), tcp.get_destination());
            println!("{}", ip.get_identification())
        }
        IpNextHeaderProtocols::Udp => {
            let udp = UdpPacket::new(ip.payload()).unwrap();
            println!("{}:{} -> {}:{}", ip.get_source(), udp.get_source(), ip.get_destination(), udp.get_destination());
            println!("{}", ip.get_identification())
        }
        IpNextHeaderProtocols::Icmp => {
            println!("{} -> {}", ip.get_source(), ip.get_destination());
        }
        _ => (),
    }
}

fn main() {
    let mut v = VecDeque::new();
    let capture_iface_name = "eth0";
    let interface_names_match = |iface: &NetworkInterface| iface.name == capture_iface_name;

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap_or_else(|| panic!("No such network interface: {}", capture_iface_name));

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("unhandled channel type: {}"),
        Err(e) => panic!("unable to create channel: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                v.push_front(
                    PacketWithInterface {
                        interface: interface.clone(),
                        packet: packet.to_owned(),
                    }
                );
                //handle_ethernet_frame(&interface, &EthernetPacket::new(packet).unwrap())
            }
            Err(_) => {
                continue;
            }
        }
    }
}