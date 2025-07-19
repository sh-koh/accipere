use std::{fs, path::Path};

#[derive(Debug)]
pub struct Os {
    pub pretty_name: String,
    pub logo: String,
    pub wm: String,
}

impl Os {
    pub fn new() -> Self {
        let (mut pretty_name, mut logo, mut wm) = (String::new(), String::new(), String::new());

        /* TODO:
         *  pretty_name: get OS name (/etc/os-release)
         *  logo: ascii art or real image (kitty image protocol) to print
         *  wm: check IPC for WM version and parse its output (possbile for Hyprland and niri)
         */

        let os_release = fs::read_to_string(Path::new("/etc/os-release"))
            .expect("Could not access `/etc/os-release`");
        for line in os_release.lines() {
            if line.starts_with("PRETTY_NAME") {
                pretty_name = line
                    .split('=')
                    .nth(1)
                    .map(|s| s.trim_matches('"').into())
                    .unwrap();
            }
            if line.starts_with("LOGO") {
                logo = line.split('=').nth(1).map(|s| s.into()).unwrap();
            }
        }

        Os {
            pretty_name,
            logo,
            wm,
        }
    }
}
