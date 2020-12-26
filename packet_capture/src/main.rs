extern crate pnet;

use pnet::datalink;

fn main() {
    let interface = datalink::interfaces();

    println!("{}", interface[2].name);
}
