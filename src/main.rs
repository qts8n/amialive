use std::fmt;
use std::collections::HashMap;
use std::net::{AddrParseError, IpAddr};
use std::process::exit;
use std::time::Duration;
use std::thread;

use rand::random;
use ping::{ping, Error as PingError};


// Exit code on error
const EXIT_CODE_ERR: i32 = 2;


// List of public DNS servers
const IP_LIST: &'static [&'static str] = &[
    "1.1.1.1",
    "8.8.8.8",
];


// Custom error enum
enum PingCheckError {
    Parse(AddrParseError),
    Ping(PingError),
}


impl PingCheckError {
    fn from_parse(err: AddrParseError) -> PingCheckError {
        PingCheckError::Parse(err)
    }

    fn from_ping(err: PingError) -> PingCheckError {
        PingCheckError::Ping(err)
    }
}


impl fmt::Display for PingCheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            PingCheckError::Parse(err) => write!(f, "PING ERROR: {err}"),
            PingCheckError::Ping(err) => write!(f, "PING ERROR: {err}"),
        }
    }
}


fn ping_ip(ip_str: &str) -> Result<(), PingCheckError> {
    let ip = ip_str.parse::<IpAddr>().map_err(PingCheckError::from_parse)?;
    ping(
        ip,
        Some(Duration::from_secs(1)),
        Some(166),
        Some(3),
        Some(5),
        Some(&random()),
    )
    .map_err(PingCheckError::from_ping)
}


fn main() {
    println!("Checking internet connection...");

    let mut handles = HashMap::new();
    for ip_str in IP_LIST {
        handles.insert(
            ip_str.to_string(),
            thread::spawn(|| {ping_ip(ip_str)})
        );
    }

    for (ip_str, handle) in handles {
        match handle.join() {
            Ok(result) => match result {
                Ok(()) => println!("{ip_str} is up"),
                Err(err) => eprintln!("{ip_str} is not responding! {err}"),
            },
            Err(_) => {
                eprintln!("Could not send ping to {ip_str}: Thread error");
                exit(EXIT_CODE_ERR);
            }
        }
    }
}
