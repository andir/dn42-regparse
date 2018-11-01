extern crate regparse;
extern crate hyper;
#[macro_use] extern crate json;

use regparse::parse::*;
use std::env;
use std::cmp;
use std::thread;
use std::sync::{Arc, Mutex};
use std::process::Command;
use std::path::PathBuf;

use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

pub struct RoaSrv {
    roa_data: String,
    parser_context: ParserContext
}

pub fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        println!("Usage: {} [REGISTRY_DATA] <REFRESH_INTERVAL>", argv[0]);
        return;
    }

    let update_interval: u32 = if argv.len() > 2 {
        argv[2].parse::<u32>().unwrap()
    } else {
        600
    };
    
    let mut data = RoaSrv {
        roa_data: String::from("{\"error\": \"The database has not been initialized yet!\"}"),
        parser_context: ParserContext::new(&argv[1], ParserConfig::routes())
    };

    data.update_roa_data();
    println!("Successfully initialized database!");

    let data = Arc::new(Mutex::new(data));
    let data_clone = Arc::clone(&data);

    thread::spawn(move || {
        let data_clone = data_clone;
        let repo_path = PathBuf::from(&argv[1]);
        
        loop {
            thread::sleep_ms(update_interval * 1000);

            let mut data = data_clone.lock().unwrap(); // Acquire lock
            
            let out = Command::new("git")
                .arg("pull")
                .current_dir(&repo_path)
                .output()
                .expect("Failed to run git pull!")
                .stdout;

            if String::from_utf8_lossy(&out).contains("Already up to date.") {
                continue;
            }

            (*data).update_roa_data();
            println!("ROA data updated!");
            // Automatically drop lock
        }
    });

    let data_clone = Arc::clone(&data);

    let service = move || {
        let data = Arc::clone(&data);
        service_fn_ok(move |_: Request<Body>| {
            let data = data.lock().unwrap();
            Response::new(Body::from((*data).roa_data.clone()))
        })
    };

    let addr = ([127, 0, 0, 1], 2312).into();
    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("Server error: {}", e));

    hyper::rt::run(server);
}

impl RoaSrv {
    
    pub fn update_roa_data(&mut self) {
        let mut roas = array![];
        let data = self.parser_context.parse();

        for route in &data.routes {
            for origin in &route.origin {
                roas.push(object!{
                    "prefix" => route.route.to_string(),
                    "maxLength" => cmp::max(29, route.route.len()),
                    "asn" => format!("AS{}", &origin)
                }).expect("Failed to add json object to roa array!");
            }
        }

        for route in &data.routes6 {
            for origin in &route.origin {
                roas.push(object!{
                    "prefix" => route.route6.to_string(),
                    "maxLength" => cmp::max(64, route.route6.len()),
                    "asn" => format!("AS{}", &origin)
                }).expect("Failed to add json object to roa array!");
            }
        }

        self.roa_data = json::stringify(object!{ "roas" => roas });
    }
    
}
