use crate::engine::parse;

pub mod engine;

#[tokio::main]
pub async fn main() {
    println!("Welcome on Weber");

    let e = std::fs::read_to_string("./static/index.html").unwrap();
    println!("{}", e);

    parse(e);
}
