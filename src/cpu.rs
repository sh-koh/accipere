use std::{
    fs,
};

#[derive(Debug, Clone)]
pub struct Cpu {
    pub model_name: String,
    pub cores: u8,
    pub threads: u8,
}

impl Cpu {
    pub fn new() -> Self {
        let mut model_name = String::new();
        let mut cores = 0;
        let mut threads = 0;

        let cpuinfo: String =
            fs::read_to_string("/proc/cpuinfo").expect("Failed to access `/proc/cpuinfo`");

        let mut props: u8 = 0;
        for line in cpuinfo.lines() {
            if props == 3 {
                break;
            };
            if line.starts_with("model name") || line.starts_with("Model") {
                props += 1;
                model_name = line
                    .split(':')
                    .nth(1)
                    .unwrap_or("Unknown")
                    .trim()
                    .into();
            }
            if line.starts_with("cpu cores") {
                props += 1;
                cores = line
                    .split(':')
                    .nth(1)
                    .unwrap_or("0")
                    .trim()
                    .parse::<u8>()
                    .unwrap_or(0);
            }
            if line.starts_with("siblings") {
                props += 1;
                threads = line
                    .split(':')
                    .nth(1)
                    .unwrap_or("0")
                    .trim()
                    .parse::<u8>()
                    .unwrap_or(0);
            }
        }
        Cpu {
            model_name,
            cores,
            threads,
        }
    }
}
