use clap::Parser;
use rust::options::Options;

fn main() {
    let opts = Options::parse();
    println!("{:?}", opts)
}
