use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::{fmt::Write, fs, path};

#[derive(Debug)]
pub struct Net {
    pub hostname: String,
    pub interfaces: Vec<NetworkInterface>,
}

impl Net {
    pub fn new() -> Self {
        let hostname = fs::read_to_string(path::Path::new("/etc/hostname"))
            .expect("Failed to get hostname")
            .trim_end()
            .to_string();
        let mut interfaces = NetworkInterface::show()
            .expect("Failed to list network interfaces")
            .into_iter()
            .filter(|iface| iface.name != "lo")
            .filter_map(|mut iface| {
                iface.addr.retain(|addr| addr.ip().is_ipv4());
                (!iface.addr.is_empty()).then_some(iface)
            })
            .collect::<Vec<NetworkInterface>>();
        interfaces.sort_by(|a, b| a.name.cmp(&b.name));

        Net {
            hostname,
            interfaces,
        }
    }

    pub fn print_all_formated(&self) -> String {
        let mut result = String::new();
        let mut acc = 0;
        for iface in &self.interfaces {
            let mut addrs = String::new();
            for addr in &iface.addr {
                write!(addrs, "{}", &addr.ip()).unwrap();
                if acc <= iface.addr.len() {
                    acc += 1;
                    write!(addrs, "{}", "\n").unwrap();
                }
            }
            write!(result, "IP({}): {}", &iface.name, addrs).unwrap();
        }
        result
    }
}
