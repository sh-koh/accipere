use ansi_term::Color::Red;
use std::thread;

mod cpu;
mod gpu;
mod net;
mod os;
mod ram;

#[derive(Debug)]
struct Accipere {
    cpu: cpu::Cpu,
    gpu: gpu::Gpu,
    net: net::Net,
    ram: ram::Ram,
    os: os::Os,
}

impl Accipere {
    pub fn new() -> Self {
        let (cpu_handle, gpu_handle, net_handle, ram_handle, os_handle) = (
            thread::spawn(|| cpu::Cpu::new()),
            thread::spawn(|| gpu::Gpu::new()),
            thread::spawn(|| net::Net::new()),
            thread::spawn(|| ram::Ram::new()),
            thread::spawn(|| os::Os::new()),
        );

        let (cpu, gpu, net, ram, os) = (
            cpu_handle
                .join()
                .expect("Thread to initilize CPU module failed"),
            gpu_handle
                .join()
                .expect("Thread to initilize GPU module failed"),
            net_handle
                .join()
                .expect("Thread to initilize NET module failed"),
            ram_handle
                .join()
                .expect("Thread to initilize RAM module failed"),
            os_handle
                .join()
                .expect("Thread to initilize OS module failed"),
        );

        Accipere {
            cpu,
            gpu,
            net,
            ram,
            os,
        }
    }

    pub fn formated(&self) -> String {
        format!(
            "Hostname: {}
OS: {}
CPU: {} ({}c/{}t)
GPU: {}
RAM: {:.2} Gib / {:.2} Gib ({:.2}%)
{}",
            &self.net.hostname,
            self.os.pretty_name,
            self.cpu.model_name,
            self.cpu.cores,
            self.cpu.threads,
            self.gpu.model,
            (self.ram.total as f32 - self.ram.available as f32) / 1_024.0 / 1_024.0,
            self.ram.total as f32 / 1_024.0 / 1_024.0,
            (self.ram.total as f32 - self.ram.available as f32) / self.ram.total as f32 * 100.0,
            self.net.print_all_formated()
        )
    }
}

fn main() {
    let accipere = Accipere::new();

    // TODO: Implement a "pretty_print" method for all properties/custom types
    println!("{}", accipere.formated());
}
