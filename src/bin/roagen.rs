extern crate regparse;

use regparse::parse::*;
use std::cmp;

pub fn main() {
    let mut context = ParserContext::new("../registry/data", ParserConfig::routes());
    let data = context.parse();

    for route in &data.routes {
        for origin in &route.origin {
            println!("route {} max {} as {};", route.route, cmp::max(29, route.route.len()), origin);
        }
    }

    for route in &data.routes6 {
        for origin in &route.origin {
            println!("route {} max {} as {};", route.route6, cmp::max(64, route.route6.len()), origin);
        }
    }
}
