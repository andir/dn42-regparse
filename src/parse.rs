use super::data::*;

use std::path::PathBuf;
use std::collections::VecDeque;

extern crate regex;
use self::regex::Regex;

lazy_static! {
    static ref AUT_NUM_FORMAT: Regex = Regex::new(r"AS(\d+)").unwrap();
}

pub struct ParserContext {
    data_path: PathBuf,
    registry_data: RegistryData,
    jobs: VecDeque<ParserJob>,
    config: ParserConfig
}

pub struct ParserConfig {
    aut_nums: bool,
    inet_nums: bool,
    inet6_nums: bool,
    as_sets: bool,
    as_blocks: bool,
    domains: bool,
    routes: bool,
    routes6: bool,
    route_sets: bool
}

impl ParserConfig {
    pub fn new() -> ParserConfig {
        ParserConfig {
            aut_nums: false,
            inet_nums: false,
            inet6_nums: false,
            as_sets: false,
            as_blocks: false,
            domains: false,
            routes: false,
            routes6: false,
            route_sets: false
        }
    }

    pub fn custom(
        par_aut_nums: bool,
        par_inet_nums: bool,
        par_inet6_nums: bool,
        par_as_sets: bool,
        par_as_blocks: bool,
        par_domains: bool,
        par_routes: bool,
        par_routes6: bool,
        par_route_sets: bool) -> ParserConfig {
        ParserConfig {
            aut_nums: par_aut_nums,
            inet_nums: par_inet_nums,
            inet6_nums: par_inet6_nums,
            as_sets: par_as_sets,
            as_blocks: par_as_blocks,
            domains: par_domains,
            routes: par_routes,
            routes6: par_routes6,
            route_sets: par_route_sets
        }
    }

    pub fn routes() -> ParserConfig {
        let mut config = ParserConfig::new();
        config.routes = true;
        config.routes6 = true;
        config
    }

    pub fn inet_nums() -> ParserConfig {
        let mut config = ParserConfig::new();
        config.inet_nums = true;
        config.inet6_nums = true;
        config
    }

    pub fn all() -> ParserConfig {
        ParserConfig::custom(true, true, true, true, true, true, true, true, true)
    }
}

enum ParserJob {
    AutNum(u32),
    InetNum(InetCidr),
    Inet6Num(Inet6Cidr),
    AsSet(String),
    AsBlock(String),
    Domain(String),
    Route(InetCidr),
    Route6(Inet6Cidr),
    RouteSet(String)
}

impl ParserContext {
    pub fn new(path: &str, config: ParserConfig) -> ParserContext {
        ParserContext {
            data_path: PathBuf::from(&path),
            registry_data: RegistryData::new(),
            jobs: VecDeque::new(),
            config: config
        }
    }

    pub fn parse(&mut self) -> &RegistryData {
        self.registry_data.clear();
        self.populate_queue();
        
        loop {
            if let Some(job) = self.jobs.pop_back() {
                match job {
                    ParserJob::AutNum(num) => self.registry_data.aut_nums.push(RegistryAutNum::from(&self.data_path, num)),
                    ParserJob::InetNum(cidr) => self.registry_data.inet_nums.push(RegistryInetNum::from(&self.data_path, cidr)),
                    ParserJob::Inet6Num(cidr) => self.registry_data.inet6_nums.push(RegistryInet6Num::from(&self.data_path, cidr)),
                    ParserJob::AsSet(name) => self.registry_data.as_sets.push(RegistryAsSet::from(&self.data_path, &name)),
                    ParserJob::AsBlock(name) => self.registry_data.as_blocks.push(RegistryAsBlock::from(&self.data_path, &name)),
                    ParserJob::Domain(name) => self.registry_data.dns.push(RegistryDns::from(&self.data_path, &name)),
                    ParserJob::Route(cidr) => self.registry_data.routes.push(RegistryRoute::from(&self.data_path, cidr)),
                    ParserJob::Route6(cidr) => self.registry_data.routes6.push(RegistryRoute6::from(&self.data_path, cidr)),
                    ParserJob::RouteSet(name) => self.registry_data.route_sets.push(RegistryRouteSet::from(&self.data_path, name))
                }
            } else { break; }
        }

        &self.registry_data
    }

    fn iter_dir(&mut self, dir: &str, conv: &Fn(&str) -> ParserJob) -> () {
        let mut path = self.data_path.clone();
        path.push(dir);

        for entry in path.read_dir().unwrap() {
            if let Ok(entry) = entry {
                if let Some(filename) = entry.file_name().to_str() {
                    self.jobs.push_front(conv(&filename));
                }
            }
        }
    }

    fn populate_queue(&mut self) {
        if self.config.aut_nums { self.iter_dir("aut-num", &|name| {
            let caps = AUT_NUM_FORMAT.captures(name).unwrap();
            ParserJob::AutNum(caps.get(1).unwrap().as_str().parse::<u32>().unwrap())
        }); }
        if self.config.as_sets { self.iter_dir("as-set", &|name| ParserJob::AsSet(String::from(name))) }
        if self.config.as_blocks { self.iter_dir("as-block", &|name| ParserJob::AsBlock(String::from(name))) }
        if self.config.domains { self.iter_dir("dns", &|name| ParserJob::Domain(String::from(name))) }
        if self.config.inet_nums { self.iter_dir("inetnum", &|name| ParserJob::InetNum(InetCidr::from_filename(name))) }
        if self.config.inet6_nums { self.iter_dir("inet6num", &|name| ParserJob::Inet6Num(Inet6Cidr::from_filename(name))) }
        if self.config.routes { self.iter_dir("route", &|name| ParserJob::Route(InetCidr::from_filename(name))) }
        if self.config.routes6 { self.iter_dir("route6", &|name| ParserJob::Route6(Inet6Cidr::from_filename(name))) }
        if self.config.route_sets { self.iter_dir("route-set", &|name| ParserJob::RouteSet(String::from(name))) }
        
    }
    
}
