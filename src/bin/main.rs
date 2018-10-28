extern crate regparse;

fn main() {
    let mut context = regparse::parse::ParserContext::new("../registry/data");
    context.parse();
}
