use std::fmt;
use std::net::{AddrParseError, IpAddr};
use std::process::exit;
use std::time::Duration;

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
    println!("Hello, world!");

    for ip_str in IP_LIST {
        if let Err(e) = ping_ip(ip_str) {
            eprintln!("{ip_str} is not responding! {e}");
            exit(EXIT_CODE_ERR);
        }
        println!("{ip_str} is up")
    }
}
