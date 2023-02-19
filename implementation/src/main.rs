mod arg_parsing;

use clap::Parser;

fn main() {
    println!("{:#?}", arg_parsing::Cli::parse());
}
