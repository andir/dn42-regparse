extern crate regparse;

use regparse::parse::*;

fn main() {
    let mut context = ParserContext::new("../registry/data", ParserConfig::all());
    context.parse();
}
