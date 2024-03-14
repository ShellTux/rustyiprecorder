use chrono::prelude::*;
use reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let public_ip = reqwest::blocking::get("https://api.ipify.org")?.text()?;

    let current_time = Local::now();

    println!("Public IP: {}", public_ip);
    println!("Timestamp: {}", current_time);

    Ok(())
}
