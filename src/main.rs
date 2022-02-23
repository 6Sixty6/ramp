use ramp::*;
use clap::{App, Arg};
use std::{
    net::{SocketAddr, ToSocketAddrs},
};

/*
  * ramp
  * SixSixtySix
  * author: 0xlilith
*/

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    // Setting up arguments
    let param = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("target")
                .help("The target to scan")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("concurrency")
                .help("Concurrency")
                .long("concurrency")
                .short("c")
                .default_value("1002"),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Display detailed information")
                .long("verbose")
                .short("v"),
        )
        .arg(
            Arg::with_name("full")
                .help("Scan all 65535 ports")
                .long("full")
                .short("f"),
        )
        .arg(
            Arg::with_name("timeout")
                .help("Connection timeout")
                .long("timeout")
                .short("t")
                .default_value("3"),
        )
        .arg(
            Arg::with_name("banner")
                .help("show banner")
                .long("banner")
                .short("b"),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    // Parsing out arguments
    let banner = param.is_present("banner");
    let full = param.is_present("full");
    let verbose = param.is_present("verbose");
    let concurrency = param
        .value_of("concurrency")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(1002);
    let timeout = param
        .value_of("timeout")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(3);
    let target = param.value_of("target").unwrap();

    if banner {
        println!("{}", BANNER); // Banner
    }

    // Checks for verbose
    if verbose {
        let ports = if full {
            String::from("all 65535 ports")
        } else {
            String::from("common 1002 ports")
        };
        println!(
            "Scanning {} of {}. Concurrency: {:?}. Timeout: {:?}",
            &ports, target, concurrency, timeout
        );
    }

    // Socket Address from the target 
    let addr: Vec<SocketAddr> = format!("{}:0", target).to_socket_addrs()?.collect();
    if addr.is_empty() {
        return Err(anyhow::anyhow!("Socket_addresses list is empty"));
    }

    send_scan(addr[0].ip(), full, concurrency, timeout).await;

    Ok(())
}


