use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};
use futures::{stream, StreamExt};
use tokio::net::TcpStream;

mod ports;

/*
  * ramp library
  * SixSixtySix
  * author: 0xlilith
*/

pub const BANNER: &str = r"           
▄▄▄   ▄▄▄· • ▌ ▄ ·.  ▄▄▄·
▀▄ █·▐█ ▀█ ·██ ▐███▪▐█ ▄█
▐▀▀▄ ▄█▀▀█ ▐█ ▌▐▌▐█· ██▀·
▐█•█▌▐█ ▪▐▌██ ██▌▐█▌▐█▪·•
.▀  ▀ ▀  ▀ ▀▀  █▪▀▀▀.▀                   
    ramp by SixSyxtySix
    https://github.com/6Sixty6/ramp
---------------------------------
";

pub async fn send_scan(target: IpAddr, full: bool, concurrency: usize, timeout: u64) {
    let ports = stream::iter(ports(full));

    ports
        .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout))
        .await;
}

pub async fn scan_port(target: IpAddr, port: u16, timeout: u64) {
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target.clone(), port);

    match tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        Ok(Ok(_)) => println!("{}", port),
        _ => {}
    }
}

// Itterating through ports
pub fn ports(full: bool) -> Box<dyn Iterator<Item = u16>> {
    if full {
        Box::new((1..=u16::MAX).into_iter())
    } else {
        Box::new(ports::COMMON_PORTS.to_owned().into_iter())
    }
}