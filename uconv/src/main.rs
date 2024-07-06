mod uconv;

use uconv::{run, Args};
use clap::Parser;

fn main() {
    let args = Args::parse();
    run(args);
}
