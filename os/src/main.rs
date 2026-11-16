use std::env;
use std::process::Command;

fn main() {
    let bios_path = env!("BIOS_PATH");

    println!("Starting Rust Micro-OS in QEMU...");
    println!("Disk image: {}", bios_path);

    let status = Command::new("qemu-system-x86_64")
        .arg("-drive")
        .arg(format!("format=raw,file={}", bios_path))
        .arg("-serial")
        .arg("stdio")
        .arg("-display")
        .arg("none")
        .arg("-no-reboot")
        .status()
        .expect("Could not start QEMU");

    if !status.success() {
        eprintln!("QEMU exited with an error: {}", status);
    }
}