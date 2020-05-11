extern crate regparse;

use regparse::data::RegistryData;
use regparse::parse::*;
use std::env;
use std::io::Write;

const IPV4_PREFIX_LEN_MAX: u8 = 28;
const IPV4_PREFIX_LEN_MIN: u8 = 15;
const IPV6_PREFIX_LEN_MAX: u8 = 64;
const IPV6_PREFIX_LEN_MIN: u8 = 40;

#[inline]
fn clamp(min: u8, max: u8, val: u8) -> u8 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

fn write_roa(data: &RegistryData, path: &std::path::PathBuf) -> std::io::Result<()> {
    let mut fh = std::fs::File::create(path)?;
    for route in &data.routes {
        for origin in &route.origin {
            let max_prefix_len = if origin == "AS0" {
                32
            } else {
                IPV4_PREFIX_LEN_MAX
            };
            writeln!(
                fh,
                "route {} max {} as {};",
                route.route,
                clamp(IPV4_PREFIX_LEN_MIN, IPV4_PREFIX_LEN_MAX, route.route.len()),
                origin
            )?;
        }
    }
    Ok(())
}

fn write_roa6(data: &RegistryData, path: &std::path::PathBuf) -> std::io::Result<()> {
    let mut fh = std::fs::File::create(path)?;
    for route in &data.routes {
        for origin in &route.origin {
            let max_prefix_len = if origin == "AS0" {
                128
            } else {
                IPV6_PREFIX_LEN_MAX
            };
            writeln!(
                fh,
                "route {} max {} as {};",
                route.route,
                clamp(IPV6_PREFIX_LEN_MIN, IPV6_PREFIX_LEN_MAX, route.route.len()),
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
