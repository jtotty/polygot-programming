use clap::Parser;
use rust::opts::Options;

fn main() {
    let opts = Options::parse();
    println!("{:?}", opts)
}
