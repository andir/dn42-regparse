use super::data::*;

use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
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
    AUT_NUM(u32)
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
        self.jobs.push_front(ParserJob::AUT_NUM(4242421191));
        
        loop {
            if let Some(job) = self.jobs.pop_back() {
                match job {
                    ParserJob::AUT_NUM(num) => self.registry_data.aut_nums.push(RegistryAutNum::from(&self.data_path, num))
                }
            } else { break; }
        }

        for x in &self.registry_data.aut_nums {
            println!("{:?}", x);
        }

        Ok(())
    }
}
