extern crate regparse;

use regparse::data::RegistryData;
use regparse::parse::*;
use std::env;
use std::io::Write;

const IPV4_PREFIX_LEN_MAX: u8 = 28;
const IPV6_PREFIX_LEN_MAX: u8 = 64;

fn write_roa(data: &RegistryData, path: &std::path::PathBuf) -> std::io::Result<()> {
    let mut fh = std::fs::File::create(path)?;
    for route in &data.routes {
        for origin in &route.origin {
            let max_prefix_len = if origin == "0" {
                32
            } else {
                IPV4_PREFIX_LEN_MAX
            };

            if route.route.len() > max_prefix_len {
                continue;
            }

            let route_max_length = route.max_length.unwrap_or(route.route.len());

            if route_max_length > route.route.len() {
                continue;
            }

            if route_max_length > 32 {
                continue;
            }

            writeln!(
                fh,
                "route {} max {} as {};",
                route.route,
                route_max_length,
                origin
            )?;
        }
    }
    Ok(())
}

fn write_roa6(data: &RegistryData, path: &std::path::PathBuf) -> std::io::Result<()> {
    let mut fh = std::fs::File::create(path)?;
    for route in &data.routes6 {
        for origin in &route.origin {
            let max_prefix_len = if origin == "0" {
                128
            } else {
                IPV6_PREFIX_LEN_MAX
            };

            if route.route6.len() > max_prefix_len {
                continue;
            }

            writeln!(
                fh,
                "route {} max {} as {};",
                route.route6,
                route.max_length.unwrap_or(route.route6.len()),
                origin
            )?;
        }
    }
    Ok(())
}

pub fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [REGISTRY_DATA]", args[0]);
        return Ok(());
    }

    let mut context = ParserContext::new(&args[1], ParserConfig::routes());
    let data = context.parse();

    let path = std::path::PathBuf::from(String::from("."));

    write_roa(&data, &path.join("roa.txt"))?;
    write_roa6(&data, &path.join("roa6.txt"))?;

    Ok(())
}
