mod lib;

use clap::{App, Arg};
use lib::parse;

const VERSION: &str = "v0.0.1";
const APPNAME: &str = "rustdice";
const AUTHOR: &str = "Zhenhui Xie <xiezh0831@yahoo.co.jp>";
const ABOUT: &str = "Dice program written in rust";

fn main() {
    let matches = App::new(APPNAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(Arg::with_name("INPUT")
             .help("input string"))
        .get_matches();

    if let Some(input) = matches.value_of("INPUT") {
        let dice_expr = parse(input);
        println!("{:?}", dice_expr);
    }
}
