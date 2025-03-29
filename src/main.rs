use std::{
    net::{SocketAddrV4, UdpSocket},
    str::FromStr,
    thread,
    time::Duration,
};

use clap::Parser;
use log::{error, info, warn};
use rosc::{OscMessage, OscPacket, OscType, address::verify_address, encoder};

/// Send one or many messages to a UDP-based OSC-accepting socket
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// OSC address to send to
    address: String,

    /// Port to send to
    port: u16,

    /// Send an integer
    #[arg(short, long)]
    int: Option<i32>,

    /// Send a float
    #[arg(short, long)]
    float: Option<f32>,

    /// Send a bool
    #[arg(short, long)]
    bool: Option<bool>,

    /// Send a string
    #[arg(short, long)]
    string: Option<String>,

    /// Amount of messages
    #[arg(short, long, default_value_t = 1)]
    count: u128,

    /// Delay between messages in ms
    #[arg(short, long, default_value_t = 1)]
    delay: u128,
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Cli::parse();

    // validate address
    match verify_address(&args.address) {
        Ok(_) => (),
        Err(e) => {
            error!("address seems to be invalid: {}", e);
            std::process::exit(1);
        }
    }

    let to_addr = match SocketAddrV4::from_str(format!("127.0.0.1:{}", &args.port).as_str()) {
        Ok(addr) => addr,
        Err(e) => {
            error!("port seems to be invalid: {}", e);
            std::process::exit(1);
        }
    };

    let sock = match UdpSocket::bind("127.0.0.1:0") {
        Ok(sock) => sock,
        Err(e) => {
            error!("failed to bind socket: {}", e);
            std::process::exit(1);
        }
    };

    let buffer = match encoder::encode(&OscPacket::Message(OscMessage {
        addr: args.address.clone(),
        args: make_args(&args),
    })) {
        Ok(buffer) => buffer,
        Err(e) => {
            error!("failed to create ODC message buffer: {}", e);
            std::process::exit(1);
        }
    };

    for i in 0..args.count {
        info!("sending message to {} via port {}", args.address, args.port);
        match sock.send_to(&buffer, to_addr) {
            Err(e) => {
                warn!("failed to send message: {}", e);
            }
            _ => {}
        }

        if i + 1 < args.count {
            thread::sleep(Duration::from_millis(args.delay as u64));
        }
    }
}


/// Create OscType vector based on cli arguments
fn make_args(args: &Cli) -> Vec<OscType> {
    if let Some(value) = args.int {
        return vec![OscType::Int(value)];
    }

    if let Some(value) = args.float {
        return vec![OscType::Float(value)];
    }

    if let Some(value) = args.bool {
        return vec![OscType::Bool(value)];
    }

    if let Some(value) = &args.string {
        return vec![OscType::String(value.into())];
    }

    vec![OscType::String("Hello world!".into())]
}
