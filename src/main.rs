
extern crate pnet;

use pnet::datalink::{self, NetworkInterface};

fn main() {


    let interfaces = datalink::interfaces();
    //for interface in interfaces {
    //    println!("{}", interface);

    //}


    println!("======");
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == "eth0";
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();

    println!("{}", interface);
    println!("Hello, world!");
}
