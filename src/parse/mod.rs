use super::data::*;

use std::path::PathBuf;
use std::collections::HashMap;
use std::collections::VecDeque;

extern crate regex;
use self::regex::Regex;

pub struct ParserContext {
    data_path: PathBuf,
    registry_data: RegistryData,
    jobs: VecDeque<ParserJob>
}

enum ParserJob {
    AutNum(u32),
    InetNum(InetCidr),
    Inet6Num(Inet6Cidr),
    AsSet(String),
    AsBlock(String),
    Domain(String),
    Route(InetCidr),
    Route6(Inet6Cidr)
}

impl ParserContext {
    pub fn new(path: &str) -> ParserContext {
        ParserContext {
            data_path: PathBuf::from(&path),
            registry_data: RegistryData::new(),
            jobs: VecDeque::new()
        }
    }

    pub fn parse(&mut self) -> ::std::io::Result<()> {
        // self.jobs.push_front(ParserJob::AutNum(4242421191));
        // self.jobs.push_front(ParserJob::InetNum(InetCidr::from("172.20.20.64", 28)));
        // self.jobs.push_front(ParserJob::Inet6Num(Inet6Cidr::new("fd42:c01d:beef::", 48)));
        // self.jobs.push_front(ParserJob::Domain(String::from("yamakaja.dn42")));
        // self.jobs.push_front(ParserJob::Route(InetCidr::from("172.20.0.53", 32)));

        let mut dns = self.data_path.clone();
        dns.push("dns");

        for entry in dns.read_dir().unwrap() {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    self.jobs.push_front(ParserJob::Domain(String::from(name)));
                }
            }
        }
        
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
                    ParserJob::Route6(cidr) => self.registry_data.routes6.push(RegistryRoute6::from(&self.data_path, cidr))
                }
            } else { break; }
        }

        // for x in &self.registry_data.aut_nums { println!("{:?}", x); }
        // for x in &self.registry_data.inet_nums { println!("{:?}", x); }
        // for x in &self.registry_data.inet6_nums { println!("{:?}", x); };
        // for x in &self.registry_data.dns { println!("{:?}", x); };
        // for x in &self.registry_data.routes { println!("{:?}", x); };
        // for x in &self.registry_data.as_blocks { println!("{:?}", x); };
        // for x in &self.registry_data.as_sets { println!("{:?}", x); };
        
        println!("{} dns entries!", &self.registry_data.dns.len());
        
        Ok(())
    }
}
