use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::vec::Vec;

extern crate regex;
use self::regex::Regex;

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"^(([a-z0-9\-]+):)?\s+(\S.*)?$").unwrap();
    static ref INETv4_FORMAT: Regex =
        Regex::new(r"(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})").unwrap();
    static ref CIDR_FILENAME_FORMAT: Regex =
        Regex::new(r"((\d{1,3}(\.\d{1,3}){3})|([:0-9a-f]+))_(\d{1,3})").unwrap();
}

fn parse<T>(
    obj: T,
    base_path: &PathBuf,
    obj_type: &str,
    obj_name: &str,
    parser: &dyn Fn(&mut T, (&str, String)),
) -> T {
    let mut obj = obj;
    let mut path = base_path.clone();
    path.push(obj_type);
    path.push(obj_name);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut last_key: Option<String> = None;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with('%') || line.is_empty() {
            continue;
        }

        let line = LINE_REGEX
            .captures(&line)
            .expect(&format!("Invalid line encountered: \"{}\"", line));
        let (key, value) = (line.get(2), line.get(3));

        let key = match key {
            Some(m) => String::from(m.as_str()),
            None => last_key.unwrap(),
        };

        let value = match value {
            Some(x) => x.as_str(),
            None => "",
        };

        if !key.starts_with("x-") {
            parser(&mut obj, (&key, String::from(value)));
        }

        last_key = Some(key);
    }

    obj
}

fn combine(a: &String, b: String) -> String {
    if a.is_empty() {
        b
    } else {
        format!("{}\n{}", &a, &b)
    }
}

fn combine_option(a: &Option<String>, b: String) -> Option<String> {
    match a {
        Some(v) => Some(format!("{}\n{}", v, b)),
        None => Some(b),
    }
}

#[derive(Debug)]
pub struct RegistryAutNum {
    pub aut_num: u32,
    pub as_name: String,
    pub descr: Option<String>,
    pub mnt_by: Vec<String>,
    pub member_of: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub org: Option<String>,
    pub import: Vec<String>,
    pub export: Vec<String>,
    pub default: Vec<String>,
    pub mp_peer: Vec<String>,
    pub mp_group: Vec<String>,
    pub mp_import: Vec<String>,
    pub mp_export: Vec<String>,
    pub mp_default: Vec<String>,
    pub geo_loc: Vec<String>, // Format: "> [lat-c] [long-c] [name]"
    pub remarks: Vec<String>,
    pub source: String,
}

impl RegistryAutNum {
    pub fn new(num: u32) -> RegistryAutNum {
        RegistryAutNum {
            aut_num: num,
            as_name: String::from(""),
            descr: None,
            mnt_by: Vec::new(),
            member_of: Vec::new(),
            admin_c: Vec::new(),
            tech_c: Vec::new(),
            org: None,
            import: Vec::new(),
            export: Vec::new(),
            default: Vec::new(),
            mp_peer: Vec::new(),
            mp_group: Vec::new(),
            mp_import: Vec::new(),
            mp_export: Vec::new(),
            mp_default: Vec::new(),
            geo_loc: Vec::new(),
            remarks: Vec::new(),
            source: String::from(""),
        }
    }
    pub fn from(base_path: &PathBuf, num: u32) -> RegistryAutNum {
        parse(
            RegistryAutNum::new(num),
            base_path,
            "aut-num",
            &format!("AS{}", num),
            &|obj, (key, value)| match key {
                "aut-num" => {
                    if format!("AS{}", num) != value {
                        panic!("Missmatching autnums: {} != AS{}", value, num);
                    }
                }
                "as-name" => obj.as_name = combine(&obj.as_name, value),
                "descr" => obj.descr = combine_option(&obj.descr, value),
                "mnt-by" => obj.mnt_by.push(value),
                "member-of" => obj.member_of.push(value),
                "admin-c" => obj.admin_c.push(value),
                "tech-c" => obj.tech_c.push(value),
                "org" => obj.org = combine_option(&obj.org, value),
                "import" => obj.import.push(value),
                "export" => obj.export.push(value),
                "default" => obj.default.push(value),
                "mp-peer" => obj.mp_peer.push(value),
                "mp-group" => obj.mp_group.push(value),
                "mp-import" => obj.mp_import.push(value),
                "mp-export" => obj.mp_export.push(value),
                "mp-default" => obj.mp_default.push(value),
                "geo-loc" => obj.geo_loc.push(value),
                "remarks" => obj.remarks.push(value),
                "source" => obj.source = value,
                _ => panic!("Unhandled entry in aut-num AS{}: {} = {}", num, key, value),
            },
        )
    }
}

#[derive(Debug)]
pub struct RegistryAsSet {
    pub as_set: String,
    pub descr: Option<String>,
    pub mnt_by: Vec<String>,
    pub members: Vec<String>,
    pub mbrs_by_ref: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub remarks: Vec<String>,
    pub source: String,
}

impl RegistryAsSet {
    pub fn new(name: &str) -> RegistryAsSet {
        RegistryAsSet {
            as_set: String::from(name),
            descr: None,
            mnt_by: Vec::new(),
            members: Vec::new(),
            mbrs_by_ref: Vec::new(),
            admin_c: Vec::new(),
            tech_c: Vec::new(),
            remarks: Vec::new(),
            source: String::from(""),
        }
    }

    pub fn from(base_path: &PathBuf, name: &str) -> RegistryAsSet {
        parse(
            RegistryAsSet::new(name),
            base_path,
            "as-set",
            name,
            &|obj, (key, value)| match key {
                "as-set" => {
                    if name != value {
                        panic!("Missmatching as-set name: {} != {}", name, value);
                    }
                }
                "descr" => obj.descr = combine_option(&obj.descr, value),
                "mnt-by" => obj.mnt_by.push(value),
                "members" => obj.members.push(value),
                "mbrs-by-ref" => obj.mbrs_by_ref.push(value),
                "admin-c" => obj.admin_c.push(value),
                "tech-c" => obj.tech_c.push(value),
                "remarks" => obj.remarks.push(value),
                "source" => obj.source = value,
                _ => panic!("Unhandled entry in as-set {}: {} = {}", name, key, value),
            },
        )
    }
}

#[derive(Debug)]
pub enum RegistryAsBlockPolicy {
    Open,
    Ask,
    Closed,
}

#[derive(Debug)]
pub struct RegistryAsBlock {
    pub as_block: String,
    pub descr: Option<String>,
    pub policy: String,
    pub mnt_by: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub remarks: Vec<String>,
    pub source: String,
}

impl RegistryAsBlock {
    pub fn new(name: &str) -> RegistryAsBlock {
        RegistryAsBlock {
            as_block: String::from(name),
            descr: None,
            policy: String::new(),
            mnt_by: Vec::new(),
            admin_c: Vec::new(),
            tech_c: Vec::new(),
            remarks: Vec::new(),
            source: String::from(""),
        }
    }

    pub fn from(base_path: &PathBuf, name: &str) -> RegistryAsBlock {
        parse(
            RegistryAsBlock::new(name),
            base_path,
            "as-block",
            name,
            &|obj, (key, value)| match key {
                "as-block" => {
                    if value != name {
                        panic!("Missmatching as-block name: {} != {}", name, value);
                    }
                }
                "descr" => obj.descr = combine_option(&obj.descr, value),
                "policy" => obj.policy = value,
                "mnt-by" => obj.mnt_by.push(value),
                "admin-c" => obj.admin_c.push(value),
                "tech-c" => obj.tech_c.push(value),
                "remarks" => obj.remarks.push(value),
                "source" => obj.source = value,
                _ => panic!("Unhandled entry in as-block {}: {} = {}", name, key, value),
            },
        )
    }
}

#[derive(Debug)]
pub struct RegistryDns {
    pub domain: String,
    pub nserver: Vec<String>,
    pub status: String, // Format: > {EVALPEND|CONNECT} [timestamp]
    pub descr: Option<String>,
    pub mnt_by: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub org: Vec<String>,
    pub country: Option<String>,
    pub remarks: Vec<String>,
    pub source: String,
}

impl RegistryDns {
    pub fn new(domain: &str) -> RegistryDns {
        RegistryDns {
            domain: String::from(domain),
            nserver: Vec::new(),
            status: String::from(""),
            descr: None,
            mnt_by: Vec::new(),
            admin_c: Vec::new(),
            tech_c: Vec::new(),
            org: Vec::new(),
            country: None,
            remarks: Vec::new(),
            source: String::from(""),
        }
    }
    pub fn from(base_path: &PathBuf, name: &str) -> RegistryDns {
        parse(RegistryDns::new(name), base_path, "dns", name, &|obj,
                                                                (
            key,
            value,
        )| {
            match key {
                "domain" => {
                    if value != name {
                        panic!("Missmatching domain names: {} != {}", name, value);
                    }
                }
                "nserver" => obj.nserver.push(value),
                "status" => obj.status = value,
                "descr" => obj.descr = combine_option(&obj.descr, value),
                "mnt-by" => obj.mnt_by.push(value),
                "admin-c" => obj.admin_c.push(value),
                "tech-c" => obj.tech_c.push(value),
                "org" => obj.org.push(value),
                "country" => obj.country = combine_option(&obj.country, value),
                "remarks" => obj.remarks.push(value),
                "source" => obj.source = value,
                _ => panic!("Unhandled entry in domain {}: {} = {}", name, key, value),
            }
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct InetNum(u8, u8, u8, u8);
#[derive(Clone, Debug)]
pub struct Inet6Num(String);
#[derive(Copy, Clone, Debug)]
pub struct InetCidr(InetNum, u8);
#[derive(Clone, Debug)]
pub struct Inet6Cidr(Inet6Num, u8);

impl InetCidr {
    pub fn new((a, b, c, d): (u8, u8, u8, u8), length: u8) -> InetCidr {
        InetCidr(InetNum(a, b, c, d), length)
    }

    pub fn from(i: &str, length: u8) -> InetCidr {
        InetCidr(InetNum::from(i), length)
    }

    pub fn from_filename(name: &str) -> InetCidr {
        let caps = CIDR_FILENAME_FORMAT
            .captures(name)
            .expect(&format!("Failed to parse filename cidr: {}", name));
        InetCidr::from(
            caps.get(1).unwrap().as_str(),
            caps.get(5).unwrap().as_str().parse::<u8>().unwrap(),
        )
    }

    pub fn len(&self) -> u8 {
        self.1
    }
}

impl Inet6Cidr {
    pub fn new(input: &str, len: u8) -> Inet6Cidr {
        Inet6Cidr(Inet6Num(String::from(input)), len)
    }

    pub fn from_filename(name: &str) -> Inet6Cidr {
        let caps = CIDR_FILENAME_FORMAT
            .captures(name)
            .expect(&format!("Failed to parse filename cidr6: {}", name));
        Inet6Cidr::new(
            caps.get(1).unwrap().as_str(),
            caps.get(5).unwrap().as_str().parse::<u8>().unwrap(),
        )
    }

    pub fn len(&self) -> u8 {
        self.1
    }
}

impl fmt::Display for InetNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let InetNum(a, b, c, d) = self;
        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}

impl fmt::Display for InetCidr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let InetCidr(num, length) = self;
        write!(f, "{}/{}", &num, length)
    }
}

impl fmt::Display for Inet6Cidr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Inet6Cidr(net, length) = self;
        write!(f, "{}/{}", net.0, length)
    }
}

impl InetNum {
    pub fn from(input: &str) -> InetNum {
        let caps = INETv4_FORMAT.captures(input).unwrap();
        let (a, b, c, d) = (
            caps.get(1).unwrap().as_str().parse::<u8>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<u8>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<u8>().unwrap(),
            caps.get(4).unwrap().as_str().parse::<u8>().unwrap(),
        );

        InetNum(a, b, c, d)
    }
}

#[derive(Debug)]
pub struct RegistryInetNumCommon {
    pub netname: String,
    pub nserver: Vec<String>,
    pub country: Vec<String>,
    pub descr: Option<String>,
    pub status: Option<String>,
    pub bgp_status: Option<String>,
    pub policy: Option<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub zone_c: Vec<String>,
    pub ds_rdata: Vec<String>,
    pub mnt_by: Vec<String>,
    pub mnt_lower: Vec<String>,
    pub mnt_routes: Vec<String>,
    pub org: Option<String>,
    pub remarks: Vec<String>,
    pub source: String,
}

impl RegistryInetNumCommon {
    pub fn new() -> RegistryInetNumCommon {
        RegistryInetNumCommon {
            netname: String::from(""),
            nserver: Vec::new(),
            country: Vec::new(),
            descr: None,
            status: None,
            bgp_status: None,
            policy: None,
            admin_c: Vec::new(),
            tech_c: Vec::new(),
            zone_c: Vec::new(),
            ds_rdata: Vec::new(),
            mnt_by: Vec::new(),
            mnt_lower: Vec::new(),
            mnt_routes: Vec::new(),
            org: None,
            remarks: Vec::new(),
            source: String::from(""),
        }
    }

    pub fn parse(&mut self, (key, value): (&str, String)) {
        match key {
            "netname" => self.netname = value,
            "nserver" => self.nserver.push(value),
            "country" => self.country.push(value),
            "descr" => self.descr = combine_option(&self.descr, value),
            "status" => self.status = combine_option(&self.status, value),
            "bgp-status" => self.bgp_status = combine_option(&self.bgp_status, value),
            "policy" => self.policy = combine_option(&self.policy, value),
            "admin-c" => self.admin_c.push(value),
            "tech-c" => self.tech_c.push(value),
            "zone-c" => self.zone_c.push(value),
            "ds-rdata" => self.ds_rdata.push(value),
            "mnt-by" => self.mnt_by.push(value),
            "mnt-lower" => self.mnt_lower.push(value),
            "mnt-routes" => self.mnt_routes.push(value),
            "org" => self.org = combine_option(&self.org, value),
            "remarks" => self.remarks.push(value),
            "source" => self.source = value,
            _ => panic!("Unhandled entry in inet(6)num: {} = {}", key, value),
        };
    }
}

#[derive(Debug)]
pub struct RegistryInetNum {
    pub inetnum: String,
    pub cidr: InetCidr,
    pub common: RegistryInetNumCommon,
}

impl RegistryInetNum {
    pub fn new(net: InetCidr) -> RegistryInetNum {
        RegistryInetNum {
            cidr: net,
            inetnum: String::from(""),
            common: RegistryInetNumCommon::new(),
        }
    }

    pub fn from(base_path: &PathBuf, cidr: InetCidr) -> RegistryInetNum {
        let InetCidr(net, length) = cidr;
        parse(
            RegistryInetNum::new(cidr),
            base_path,
            "inetnum",
            &format!("{}_{}", net, length),
            &|obj, (key, value)| match key {
                "cidr" => {
                    if value != cidr.to_string() {
                        panic!("Missmatching cidr in inetnum: '{}' != '{}'", cidr, value);
                    }
                }
                "inetnum" => obj.inetnum = value,
                _ => obj.common.parse((key, value)),
            },
        )
    }
}

#[derive(Debug)]
pub struct RegistryInet6Num {
    pub inet6num: String,
    pub cidr: Inet6Cidr,
    pub common: RegistryInetNumCommon,
}

impl RegistryInet6Num {
    pub fn new(net: Inet6Cidr) -> RegistryInet6Num {
        RegistryInet6Num {
            cidr: net,
            inet6num: String::from(""),
            common: RegistryInetNumCommon::new(),
        }
    }

    pub fn from(base_path: &PathBuf, cidr: Inet6Cidr) -> RegistryInet6Num {
        let Inet6Cidr(net, length) = cidr.clone();
        parse(
            RegistryInet6Num::new(cidr),
            base_path,
            "inet6num",
            &format!("{}_{}", net.0, length),
            &|obj, (key, value)| match key {
                "cidr" => {
                    if value != obj.cidr.to_string() {
                        panic!("Missmatching cidr in inet6num: {} != {}", obj.cidr, value);
                    }
                }
                "inet6num" => obj.inet6num = value,
                _ => obj.common.parse((key, value)),
            },
        )
    }
}

pub enum RegistryKeyCertMethod {
    PGP,
    X509,
    MTN,
}

#[derive(Debug)]
pub struct RegistryRoute {
    pub route: InetCidr,
    pub max_length: Option<u8>,
    pub mnt_by: Vec<String>,
    pub origin: Vec<String>,
    pub member_of: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub descr: Option<String>,
    pub remarks: Vec<String>,
    pub source: String,
    pub pingable: Vec<InetNum>,
}

impl RegistryRoute {
    pub fn new(cidr: InetCidr) -> RegistryRoute {
        RegistryRoute {
            route: cidr,
            max_length: None,
            mnt_by: Vec::new(),
            origin: Vec::new(),
            member_of: Vec::new(),
            admin_c: Vec::new(),
            tech_c: Vec::new(),
            descr: None,
            remarks: Vec::new(),
            source: String::from(""),
            pingable: Vec::new(),
        }
    }

    pub fn from(base_path: &PathBuf, cidr: InetCidr) -> RegistryRoute {
        let InetCidr(net, length) = cidr;
        parse(
            RegistryRoute::new(cidr),
            base_path,
            "route",
            &format!("{}_{}", net, length),
            &|obj, (key, value)| match key {
                "route" => {
                    if cidr.to_string() != value {
                        panic!(
                            "Missmatching cidr in route: {} != {}",
                            cidr.to_string(),
                            value
                        );
                    }
                }
                "max-length" => obj.max_length = Some((&value[..]).parse::<u8>().unwrap()),
                "mnt-by" => obj.mnt_by.push(value),
                "origin" => obj.origin.push(String::from(&value[2..])),
                "member-of" => obj.member_of.push(value),
                "admin-c" => obj.admin_c.push(value),
                "tech-c" => obj.tech_c.push(value),
                "descr" => obj.descr = combine_option(&obj.descr, value),
                "remarks" => obj.remarks.push(value),
                "source" => obj.source = value,
                "pingable" => obj.pingable.push(InetNum::from(&value)),
                _ => panic!("Unhandled entry in route {}: {} = {}", cidr, key, value),
            },
        )
    }
}

#[derive(Debug)]
pub struct RegistryRoute6 {
    pub route6: Inet6Cidr,
    pub max_length: Option<u8>,
    pub mnt_by: Vec<String>,
    pub origin: Vec<String>,
    pub member_of: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub descr: Vec<String>,
    pub remarks: Vec<String>,
    pub source: String,
    pub pingable: Vec<Inet6Num>,
}

impl RegistryRoute6 {
    pub fn new(cidr: Inet6Cidr) -> RegistryRoute6 {
        RegistryRoute6 {
            route6: cidr,
            max_length: None,
            mnt_by: Vec::new(),
            origin: Vec::new(),
            member_of: Vec::new(),
            admin_c: Vec::new(),
            tech_c: Vec::new(),
            descr: Vec::new(),
            remarks: Vec::new(),
            source: String::from(""),
            pingable: Vec::new(),
        }
    }

    pub fn from(base_path: &PathBuf, cidr: Inet6Cidr) -> RegistryRoute6 {
        let Inet6Cidr(Inet6Num(net), length) = cidr.clone();
        parse(
            RegistryRoute6::new(cidr.clone()),
            base_path,
            "route6",
            &format!("{}_{}", &net, length),
            &|obj, (key, value)| match key {
                "route6" => {
                    if cidr.to_string() != value {
                        panic!("Missmatch in route6: {} != {}", cidr.to_string(), value);
                    }
                }
                "max-length" => obj.max_length = Some((&value[..]).parse::<u8>().unwrap()),
                "mnt-by" => obj.mnt_by.push(value),
                "origin" => obj.origin.push(String::from(&value[2..])),
                "member-of" => obj.member_of.push(value),
                "admin-c" => obj.admin_c.push(value),
                "tech-c" => obj.tech_c.push(value),
                "descr" => obj.descr.push(value),
                "remarks" => obj.remarks.push(value),
                "source" => obj.source = value,
                "pingable" => obj.pingable.push(Inet6Num(value)),
                _ => panic!("Unhandled entry in route6 {}: {} = {}", cidr, key, value),
            },
        )
    }
}

pub struct RegistryRouteSet {
    pub route_set: String,
    pub descr: Option<String>,
    pub mnt_by: Vec<String>,
    pub members: Vec<String>,
    pub mp_members: Vec<String>,
    pub mbrs_by_ref: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub remarks: Vec<String>,
    pub source: String,
}

impl RegistryRouteSet {
    pub fn new(name: String) -> RegistryRouteSet {
        RegistryRouteSet {
            route_set: name,
            descr: None,
            mnt_by: Vec::new(),
            members: Vec::new(),
            mp_members: Vec::new(),
            mbrs_by_ref: Vec::new(),
            admin_c: Vec::new(),
            tech_c: Vec::new(),
            remarks: Vec::new(),
            source: String::from(""),
        }
    }

    pub fn from(base_path: &PathBuf, name: String) -> RegistryRouteSet {
        let name_clone = name.clone();
        parse(
            RegistryRouteSet::new(name_clone),
            base_path,
            "route-set",
            &name,
            &|obj, (key, value)| match key {
                "route-set" => {
                    if value != obj.route_set {
                        panic!(
                            "Missmatching name in route-set: {} != {}",
                            obj.route_set, value
                        );
                    }
                }
                "descr" => obj.descr = combine_option(&obj.descr, value),
                "mnt-by" => obj.mnt_by.push(value),
                "members" => obj.members.push(value),
                "mp-members" => obj.mp_members.push(value),
                "mbrs-by-ref" => obj.mbrs_by_ref.push(value),
                "admin-c" => obj.admin_c.push(value),
                "tech-c" => obj.tech_c.push(value),
                "remarks" => obj.remarks.push(value),
                "source" => obj.source = value,
                _ => panic!(
                    "Unhandled entry in route-set {}: {} = {}",
                    obj.route_set, key, value
                ),
            },
        )
    }
}

pub struct RegistryData {
    pub aut_nums: Vec<RegistryAutNum>,
    pub as_sets: Vec<RegistryAsSet>,
    pub as_blocks: Vec<RegistryAsBlock>,
    pub dns: Vec<RegistryDns>,
    pub inet_nums: Vec<RegistryInetNum>,
    pub inet6_nums: Vec<RegistryInet6Num>,
    pub routes: Vec<RegistryRoute>,
    pub routes6: Vec<RegistryRoute6>,
    pub route_sets: Vec<RegistryRouteSet>,
}

impl RegistryData {
    pub fn new() -> RegistryData {
        RegistryData {
            aut_nums: Vec::new(),
            as_sets: Vec::new(),
            as_blocks: Vec::new(),
            dns: Vec::new(),
            inet_nums: Vec::new(),
            inet6_nums: Vec::new(),
            routes: Vec::new(),
            routes6: Vec::new(),
            route_sets: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.aut_nums.clear();
        self.as_sets.clear();
        self.as_blocks.clear();
        self.dns.clear();
        self.inet_nums.clear();
        self.inet6_nums.clear();
        self.routes.clear();
        self.routes6.clear();
        self.route_sets.clear();
    }
}
