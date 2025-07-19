use std::{
    fs,
};

#[derive(Debug)]
pub struct Ram {
    pub total: u64,
    pub available: u64,
}

impl Ram {
    pub fn new() -> Self {
        let mut total = 0;
        let mut available = 0;

        let mut props: u8 = 0;
        if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
            for line in meminfo.lines() {
                if props >= 2 {
                    break;
                };
                match line {
                    line if line.starts_with("MemTotal:") => {
                        props += 1;
                        total = line
                            .split_whitespace()
                            .nth(1)
                            .unwrap_or_default()
                            .parse::<u64>()
                            .unwrap_or_default()
                    }
                    line if line.starts_with("MemAvailable:") => {
                        props += 1;
                        available = line
                            .split_whitespace()
                            .nth(1)
                            .unwrap()
                            .parse::<u64>()
                            .unwrap()
                    }
                    _ => continue,
                }
            }
        };
        Ram { total, available }
    }
}
