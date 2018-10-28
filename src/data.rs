use std::vec::Vec;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

extern crate regex;
use self::regex::Regex;

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"([a-z0-9\-]+):\s+(\S.+)").unwrap();
}

fn read(base_path: &PathBuf, t: &str, obj_name: &str) -> BufReader<File> {
    let mut path = base_path.clone();
    path.push(t);
    path.push(obj_name);
    let file = File::open(path).expect("Failed to open file!");
    BufReader::new(file)
}

pub type RouteSetIndex = usize;
pub type AsSetIndex = usize;
pub type AutNumIndex = usize;

#[derive(Debug)]
pub enum RegistryAsRouteSetMember {
    AsSet(AsSetIndex),
    RouteSet(RouteSetIndex)
}

#[derive(Debug)]
pub struct RegistryAutNum {
    pub aut_num: u32,
    pub as_name: String,
    pub descr: Option<String>,
    pub mnt_by: Vec<String>,
    pub member_of: Vec<RegistryAsRouteSetMember>,
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
    pub source: String
}

impl RegistryAutNum {

    pub fn new() -> RegistryAutNum {
        RegistryAutNum {
            aut_num: 0,
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
            source: String::from("")
        }
    }
    
    pub fn from(base_path: &PathBuf, num: u32) -> RegistryAutNum {
        let reader = read(base_path, "aut-num", &format!("AS{}", num));
        let mut obj = RegistryAutNum::new();

        obj.aut_num = num;
        
        for line in reader.lines() {
            let line = line.unwrap();
            let caps = LINE_REGEX.captures(&line).unwrap();

            let (key, value) = (caps.get(1).unwrap().as_str(), String::from(caps.get(2).unwrap().as_str()));

            match key {
                "aut-num" => if format!("AS{}", num) != value { panic!("Missmatching autnums: {} != AS{}", value, num) },
                "as-name" => obj.as_name = value,
                "descr" => obj.descr = Some(value),
                "mnt-by" => obj.mnt_by.push(value),
//                 "member-of" => obj.member_of.push(value),
                "admin-c" => obj.admin_c.push(value),
                "tech-c" => obj.tech_c.push(value),
                "org" => obj.org = Some(value),
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
                _ => println!("Unhandled entry: {} = {}", key, value)
            }
        }

        obj
    }
}

pub enum RegistryAsSetMember {
    AsNum(AutNumIndex),
    AsSet(AsSetIndex)
}

pub struct RegistryAsSet {
    pub as_set: String,
    pub descr: Option<String>,
    pub mnt_by: Vec<String>,
    pub members: Vec<RegistryAsSetMember>,
    pub mbrs_by_ref: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub remarks: Vec<String>,
    pub source: String
}

pub enum RegistryAsBlockPolicy {
    Open,
    Ask,
    Closed
}

pub struct RegistryAsBlock {
    pub as_block: String,
    pub descr: Option<String>,
    pub policy: RegistryAsBlockPolicy,
    pub mnt_by: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub remarks: Vec<String>,
    pub source: String
}

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
    pub source: String
}

pub struct InetNum(u8, u8, u8, u8);
pub struct Inet6Num(u16, u16, u16, u16, u16, u16, u16, u16);
pub struct InetCidr(InetNum, u8);
pub struct Inet6Cidr(Inet6Num, u8);

pub struct RegistryInetNumCommon {
    pub netname: String,
    pub nserver: Vec<String>,
    pub country: Vec<String>,
    pub descr: Option<String>,
    pub status: String,
    pub bgp_status: String,
    pub policy: String,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub zone_c: Vec<String>,
    pub ds_rdata: Vec<String>,
    pub mnt_by: Vec<String>,
    pub mnt_lower: Vec<String>,
    pub mnt_routers: Vec<String>,
    pub org: Option<String>,
    pub remarks: Vec<String>,
    pub source: String
}

pub struct RegistryInetNum {
    pub inetnum: InetNum,
    pub cidr: InetCidr,
    pub common: RegistryInetNumCommon
}

pub struct RegistryInet6Num {
    pub inet6num: Inet6Num,
    pub cidr: Inet6Cidr,
    pub common: RegistryInetNumCommon
}

pub enum RegistryKeyCertMethod {
    PGP,
    X509,
    MTN
}

pub struct RegistryObj {
    pub registry: String,
    pub url: Vec<String>,
    pub descr: Vec<String>,
    pub mnt_by: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub source: String
}

pub struct RegistryRoute {
    pub route: InetCidr,
    pub mnt_by: Vec<String>,
    pub origin: Vec<AutNumIndex>,
    pub member_of: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub descr: Option<String>,
    pub remarks: Vec<String>,
    pub source: String,
    pub pingable: Vec<InetNum>
}

pub struct RegistryRoute6 {
    pub route6: Inet6Cidr,
    pub mnt_by: Vec<String>,
    pub origin: Vec<AutNumIndex>,
    pub member_of: Vec<RouteSetIndex>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub descr: Vec<String>,
    pub remarks: Vec<String>,
    pub source: String,
    pub pingable: Vec<Inet6Num>
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
    pub source: String
}

pub struct RegistryTincKey {
    pub tinc_key: String,
    pub tinc_host: String,
    pub tinc_file: String,
    pub descr: Option<String>,
    pub remarks: Vec<String>,
    pub compression: Option<String>,
    pub subnet: Vec<String>,
    pub tinc_address: Option<String>,
    pub port: Option<u16>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub mnt_by: Vec<String>,
    pub source: String
}

pub struct RegistryData {
    pub aut_nums: Vec<RegistryAutNum>,
    pub as_sets: Vec<RegistryAsSet>,
    pub as_blocks: Vec<RegistryAsBlock>,
    pub dns: Vec<RegistryDns>,
    pub inet_nums: Vec<RegistryInetNum>,
    pub inet6_nums: Vec<RegistryInet6Num>,
    pub registries: Vec<RegistryObj>,
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
            registries: Vec::new(),
            routes: Vec::new(),
            routes6: Vec::new(),
            route_sets: Vec::new(),
        }
    }
}
