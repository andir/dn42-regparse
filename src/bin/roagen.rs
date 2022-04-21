extern crate regparse;

use cidr::Ipv4Cidr;
use regparse::data::RegistryData;
use regparse::parse::*;
use std::env;
use std::io::Write;
use std::net::Ipv4Addr;

lazy_static::lazy_static! {
    static ref IPV4_CIDR_172_20_0_0_24: Ipv4Cidr = Ipv4Cidr::new(Ipv4Addr::new(172, 20, 0, 0), 24).unwrap();
    static ref IPV4_CIDR_172_21_0_0_24: Ipv4Cidr = Ipv4Cidr::new(Ipv4Addr::new(172,21,0,0), 24).unwrap();
    static ref IPV4_CIDR_172_22_0_0_24: Ipv4Cidr = Ipv4Cidr::new(Ipv4Addr::new(172,22,0,0), 24).unwrap();
    static ref IPV4_CIDR_172_23_0_0_24: Ipv4Cidr = Ipv4Cidr::new(Ipv4Addr::new(172,23,0,0), 24).unwrap();
    static ref IPV4_CIDR_172_20_0_0_14: Ipv4Cidr = Ipv4Cidr::new(Ipv4Addr::new(172,20,0,0), 14).unwrap();
    static ref IPV4_CIDR_10_100_0_0_14: Ipv4Cidr = Ipv4Cidr::new(Ipv4Addr::new(10, 100, 0, 0), 14).unwrap();
    static ref IPV4_CIDR_10_127_0_0_16: Ipv4Cidr = Ipv4Cidr::new(Ipv4Addr::new(10, 127, 0, 0), 16).unwrap();
    static ref IPV4_CIDR_10_0_0_0_8: Ipv4Cidr = Ipv4Cidr::new(Ipv4Addr::new(10, 0, 0, 0), 8).unwrap();
    static ref IPV4_CIDR_172_31_0_0_16: Ipv4Cidr = Ipv4Cidr::new(Ipv4Addr::new(172,31, 0, 0), 16).unwrap();
}

fn ipv4_min_max_default_prefix_len(num: &regparse::data::InetCidr) -> (u8, u8) {
    let inet_num = std::net::Ipv4Addr::from(num.num());

    if IPV4_CIDR_172_20_0_0_24.contains(&inet_num) {
        (24, 32)
    } else if IPV4_CIDR_172_21_0_0_24.contains(&inet_num) {
        (28, 32)
    } else if IPV4_CIDR_172_22_0_0_24.contains(&inet_num) {
        (28, 32)
    } else if IPV4_CIDR_172_23_0_0_24.contains(&inet_num) {
        (28, 32)
    } else if IPV4_CIDR_172_20_0_0_14.contains(&inet_num) {
        (21, 29)
    } else if IPV4_CIDR_10_100_0_0_14.contains(&inet_num) {
        (14, 32)
    } else if IPV4_CIDR_10_127_0_0_16.contains(&inet_num) {
        (16, 32)
    } else if IPV4_CIDR_10_0_0_0_8.contains(&inet_num) {
        (15, 24)
    } else if IPV4_CIDR_172_31_0_0_16.contains(&inet_num) {
        (16, 32)
    } else {
        (0, 32)
    }
}

lazy_static::lazy_static! {
static ref IPV6_FD00__8 : cidr::Ipv6Cidr = cidr::Ipv6Cidr::new(
    std::net::Ipv6Addr::new(0xfd00, 0, 0, 0, 0, 0, 0, 0), 8
).unwrap();
}

fn ipv6_min_max_default_prefix_len(num: &regparse::data::Inet6Cidr) -> (u8, u8) {
    let inet_num: std::net::Ipv6Addr = num.num().try_into().unwrap();
    if IPV6_FD00__8.contains(&inet_num) {
        (44, 64)
    } else {
        (0, 128)
    }
}

fn write_roa(data: &RegistryData, path: &std::path::PathBuf) -> std::io::Result<()> {
    let mut fh = std::fs::File::create(path)?;
    for route in &data.routes {
        for origin in &route.origin {
            let (min_prefix_len, max_prefix_len) = if origin == "0" {
                // Anycast
                (28, 32)
            } else {
                ipv4_min_max_default_prefix_len(&route.route)
            };

            if min_prefix_len == 0 && max_prefix_len == 32 {
                // special case
                dbg!("prefix not permitted at all");
                continue;
            }

            if route.route.len() < min_prefix_len {
                dbg!(
                    "Prefix length below minimum permitted prefix size",
                    route.route.len(),
                    min_prefix_len
                );
                continue;
            }

            let route_max_length = match route.max_length {
                Some(l) if l < route.route.len() => {
                    dbg!(
                        "route_max_length is smaller than the actual prefix len",
                        l,
                        route.route.len()
                    );
                    continue;
                }
                Some(l) => l,
                None if max_prefix_len > route.route.len() => max_prefix_len,
                None => route.route.len(),
            };

            if route_max_length > max_prefix_len {
                continue;
            }

            if route_max_length > 32 {
                continue;
            }

            if route_max_length < route.route.len() {
                continue;
            }

            writeln!(
                fh,
                "route {} max {} as {};",
                route.route, route_max_length, origin
            )?;
        }
    }
    Ok(())
}

fn write_roa6(data: &RegistryData, path: &std::path::PathBuf) -> std::io::Result<()> {
    let mut fh = std::fs::File::create(path)?;
    for route in &data.routes6 {
        for origin in &route.origin {
            let (min_prefix_len, max_prefix_len) = ipv6_min_max_default_prefix_len(&route.route6);

            dbg!(min_prefix_len, max_prefix_len);
            if route.route6.len() > max_prefix_len {
                dbg!(
                    "route prefix length exceeds max prefix length",
                    route.route6.len(),
                    max_prefix_len
                );
                continue;
            }

            if route.route6.len() < min_prefix_len {
                dbg!(
                    "route prefix length shorther than permitted",
                    route.route6.len(),
                    min_prefix_len
                );
                continue;
            }

            let max_length = match route.max_length {
                Some(l) if l < route.route6.len() => {
                    dbg!(
                        "max_length is smaller than the actual prefix len",
                        l,
                        route.route6.len()
                    );
                    continue;
                }
                Some(l) => l,
                None if max_prefix_len > route.route6.len() => max_prefix_len,
                None => route.route6.len(),
            };

            if max_length > max_prefix_len {
                dbg!(
                    "max length exceeds maximum permitted prefix length",
                    max_length,
                    max_prefix_len
                );
                continue;
            }

            writeln!(
                fh,
                "route {} max {} as {};",
                route.route6, max_length, origin
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

#[cfg(test)]
mod tests {

    use super::*;
    use regparse::data::*;

    fn run_test(registry: &RegistryData) -> std::io::Result<String> {
        let dir = tempfile::tempdir()?;
        let file = dir.path().join("out.txt");
        super::write_roa(&registry, &file)?;
        let contents = std::fs::read_to_string(file)?;
        Ok(contents)
    }

    fn run_test6(registry: &RegistryData) -> std::io::Result<String> {
        let dir = tempfile::tempdir()?;
        let file = dir.path().join("out.txt");
        super::write_roa6(&registry, &file)?;
        let contents = std::fs::read_to_string(file)?;
        Ok(contents)
    }

    #[test]
    fn test_format_roa4_empty() {
        let data = super::RegistryData::new();
        let contents = run_test(&data).unwrap();
        assert_eq!(contents, "");
    }

    #[test]
    fn test_format_roa4_24_29() {
        let mut data = super::RegistryData::new();
        let mut route = RegistryRoute::new(InetCidr::from("172.20.24.0", 24));
        route.origin.push("AS123".into());
        data.routes.push(route);
        let contents = run_test(&data).unwrap();
        assert_eq!(contents, "route 172.20.24.0/24 max 29 as AS123;\n");
    }

    #[test]
    fn test_format_roa4_23_no_min_prefix_len() {
        let mut data = super::RegistryData::new();
        let mut route = RegistryRoute::new(InetCidr::from("172.20.24.0", 23));
        route.origin.push("AS123".into());
        data.routes.push(route);
        let contents = run_test(&data).unwrap();
        assert_eq!(contents, "route 172.20.24.0/23 max 29 as AS123;\n");
    }

    #[test]
    fn test_format_roa4_23_24() {
        let mut data = super::RegistryData::new();
        let mut route = RegistryRoute::new(InetCidr::from("172.20.24.0", 23));
        route.max_length = Some(24);
        route.origin.push("AS123".into());
        data.routes.push(route);
        let contents = run_test(&data).unwrap();
        assert_eq!(contents, "route 172.20.24.0/23 max 24 as AS123;\n");
    }

    #[test]
    fn test_format_roa4_24_21() {
        let mut data = super::RegistryData::new();
        let mut route = RegistryRoute::new(InetCidr::from("127.0.0.1", 24));
        route.max_length = Some(21);
        route.origin.push("AS123".into());
        data.routes.push(route);
        let contents = run_test(&data).unwrap();
        assert!(
            contents.is_empty(),
            "Expected no result but got: {:?}",
            contents
        );
    }

    #[test]
    fn test_format_roa4_25_28() {
        let mut data = super::RegistryData::new();
        let mut route = RegistryRoute::new(InetCidr::from("172.23.245.128", 25));
        route.max_length = Some(28);
        route.origin.push("AS123".into());
        data.routes.push(route);
        let contents = run_test(&data).unwrap();
        assert_eq!(contents, "route 172.23.245.128/25 max 28 as AS123;\n");
    }

    #[test]
    fn test_format_roa6_64_64() {
        let mut data = super::RegistryData::new();
        let mut route = RegistryRoute6::new(Inet6Cidr::new("fd00:1234::", 64));
        route.origin.push("AS123".into());
        data.routes6.push(route);
        let contents = run_test6(&data).unwrap();
        assert_eq!(contents, "route fd00:1234::/64 max 64 as AS123;\n");
    }

    #[test]
    fn test_format_roa6_44_48() {
        let mut data = super::RegistryData::new();
        let mut route = RegistryRoute6::new(Inet6Cidr::new("fd00:1234::", 44));
        route.max_length = Some(48);
        route.origin.push("AS123".into());
        data.routes6.push(route);
        let contents = run_test6(&data).unwrap();
        assert_eq!(contents, "route fd00:1234::/44 max 48 as AS123;\n");
    }

    #[test]
    fn test_format_roa6_44_64() {
        let mut data = super::RegistryData::new();
        let mut route = RegistryRoute6::new(Inet6Cidr::new("fd00:1234::", 44));
        //route.max_length = Some(64); // implicit due to address range
        route.origin.push("AS123".into());
        data.routes6.push(route);
        let contents = run_test6(&data).unwrap();
        assert_eq!(contents, "route fd00:1234::/44 max 64 as AS123;\n");
    }

    #[test]
    fn test_format_roa6_44_63() {
        let mut data = super::RegistryData::new();
        let mut route = RegistryRoute6::new(Inet6Cidr::new("fd00:1234::", 44));
        route.max_length = Some(63);
        route.origin.push("AS123".into());
        data.routes6.push(route);
        let contents = run_test6(&data).unwrap();
        assert_eq!(contents, "route fd00:1234::/44 max 63 as AS123;\n");
    }

    #[test]
    fn test_format_roa6_44_43() {
        let mut data = super::RegistryData::new();
        let mut route = RegistryRoute6::new(Inet6Cidr::new("fd00:1234::", 44));
        route.max_length = Some(43);
        route.origin.push("AS123".into());
        data.routes6.push(route);
        let contents = run_test6(&data).unwrap();
        assert!(contents.is_empty());
    }

    #[test]
    fn test_format_roa6_multiple_origins() {
        let mut data = super::RegistryData::new();
        let mut route = RegistryRoute6::new(Inet6Cidr::new("fd00:1234::", 44));
        route.max_length = Some(48);
        route.origin.push("AS123".into());
        route.origin.push("AS124".into());
        data.routes6.push(route);
        let contents = run_test6(&data).unwrap();
        assert_eq!(
            contents,
            "route fd00:1234::/44 max 48 as AS123;\nroute fd00:1234::/44 max 48 as AS124;\n"
        );
    }

    #[test]
    fn test_ipv4_min_max_default_prefix_len() {
        assert_eq!(
            ipv4_min_max_default_prefix_len(&InetCidr::new((1, 2, 3, 4), 1)),
            (0, 32)
        );

        assert_eq!(
            ipv4_min_max_default_prefix_len(&InetCidr::new((172, 20, 0, 1), 1)),
            (24, 32)
        );
        assert_eq!(
            ipv4_min_max_default_prefix_len(&InetCidr::new((172, 21, 0, 1), 1)),
            (28, 32)
        );
        assert_eq!(
            ipv4_min_max_default_prefix_len(&InetCidr::new((172, 22, 0, 1), 1)),
            (28, 32)
        );
        assert_eq!(
            ipv4_min_max_default_prefix_len(&InetCidr::new((172, 23, 0, 1), 1)),
            (28, 32)
        );

        assert_eq!(
            ipv4_min_max_default_prefix_len(&InetCidr::new((172, 24, 0, 1), 1)),
            (0, 32)
        );

        assert_eq!(
            ipv4_min_max_default_prefix_len(&InetCidr::new((10, 100, 0, 1), 1)),
            (14, 32)
        );

        assert_eq!(
            ipv4_min_max_default_prefix_len(&InetCidr::new((10, 127, 0, 1), 1)),
            (16, 32)
        );

        assert_eq!(
            ipv4_min_max_default_prefix_len(&InetCidr::new((10, 0, 0, 1), 1)),
            (15, 24)
        );

        assert_eq!(
            ipv4_min_max_default_prefix_len(&InetCidr::new((172, 31, 0, 0), 1)),
            (16, 32)
        );
    }

    #[test]
    fn test_ipv6_min_max_prefix_len() {
        assert_eq!(
            ipv6_min_max_default_prefix_len(&Inet6Cidr::new("ff::", 8)),
            (0, 128)
        );
        assert_eq!(
            ipv6_min_max_default_prefix_len(&Inet6Cidr::new("fd00::", 8)),
            (44, 64)
        );
        assert_eq!(
            ipv6_min_max_default_prefix_len(&Inet6Cidr::new("fd00:1234::", 8)),
            (44, 64)
        );
    }
}
