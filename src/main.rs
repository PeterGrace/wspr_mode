#[macro_use] extern crate tracing;

use std::thread::sleep;
use clap::Parser;
use console_subscriber as tokio_console_subscriber;
use tracing_subscriber::{EnvFilter, Registry, prelude::*};
use tracing_subscriber::fmt::format::FmtSpan;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
pub struct CliArgs {
    port: String,
    callsign: String,
    #[arg(short, long)]
    grid: Option<String>,
    #[arg(short, long)]
    power: Option<i32>,
    #[arg(short, long)]
    band: Option<i32>,
}

fn main() {
    let _ = dotenv::dotenv();

    //region console logging
    let console_layer = tokio_console_subscriber::spawn();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .expect("Could not create env filter for logging");
    let format_layer = tracing_subscriber::fmt::layer()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_file(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_line_number(true),
        )
        .with_span_events(FmtSpan::NONE);


    let subscriber = Registry::default()
        .with(console_layer)
        .with(filter_layer)
        .with(format_layer);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
    //endregion
    let args = CliArgs::parse();
    let band = match args.band {
        Some(band) => band,
        None => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            let total_seconds = now.as_secs();
            let minute = (total_seconds / 60) % 60;
            info!("minute is {minute}");

            match minute {
                00..=01| 20..=21| 40..=41 => 160,
                02..=03| 22..=23| 42..=43 => 80,
                04..=05| 24..=25| 44..=45 => 60,
                06..=07| 26..=27| 46..=47 => 40,
                08..=09| 28..=29| 48..=49 => 30,
                10..=11| 30..=31| 50..=51 => 20,
                12..=13| 32..=33| 52..=53 => 17,
                14..=15| 34..=35| 54..=55 => 15,
                16..=17| 36..=37| 56..=57 => 12,
                18..=19| 38..=39| 58..=59 => 10,
                _ => 6
            }
        }
    };

    let freq = match band {
        160 => 1_838_100,
        80 => 3_570_100,
        40 => 7_040_100,
        30 => 10_140_200,
        20 => 14_097_100,
        17 => 18_106_100,
        15 => 21_096_100,
        12 => 24_926_100,
        10 => 28_126_100,
        6 => 50_294_500,
        _ => {
            error!("Invalid band specified, select from [160 80 40 30 20 17 15 12 10 6]");
            std::process::exit(1);
        }

    };

    let msg = format!("CONFIG:{},{},{},{}\r\n", args.callsign, args.grid.unwrap_or("    ".to_string()), args.power.unwrap_or(23), freq);

    let mut fd = match serialport::new(args.port, 9600).timeout(std::time::Duration::from_millis(100)).open() {
        Ok(fd) => fd,
        Err(e) => {
            error!("Failed to open serial port: {}", e);
            std::process::exit(1);
        }
    };
    // wait 1 second to send config line
    sleep(std::time::Duration::from_secs(1));
    match fd.write_all(msg.as_bytes()) {
        Ok(_) => {
            let _ =fd.flush();
            info!("Wrote to serial port: {msg}");
        },
        Err(e) => {
            error!("Failed to write to serial port: {}", e);
            std::process::exit(1);
        }
    };
    // wait 1 second before attempting to read buffer
    sleep(std::time::Duration::from_secs(1));
    let mut buffer = [0u8; 1024];
    let _ = fd.read(&mut buffer);
    info!("Read from serial port: {}", String::from_utf8_lossy(&buffer));


}

