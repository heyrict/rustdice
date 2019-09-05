mod lib;

use clap::{App, Arg};
use lib::parse::parse;
use std::io;

const VERSION: &str = "v0.1.0";
const APPNAME: &str = "rustdice";
const AUTHOR: &str = "Zhenhui Xie <xiezh0831@yahoo.co.jp>";
const ABOUT: &str = "Dice program written in rust";

fn main() {
    let matches = App::new(APPNAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(Arg::with_name("INPUT").help("input string"))
        .get_matches();

    if let Some(input) = matches.value_of("INPUT") {
        let dice_expr = parse(input);
        println!("> {}", dice_expr);
        println!("Result: {}", dice_expr.go());
    } else {
        // Interactive mode
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(bytes) => {
                    if bytes == 0 {
                        break;
                    }
                    let dice_expr = parse(&input.trim_end());
                    println!("> {}", dice_expr);
                    println!("Result: {}", dice_expr.go());
                }
                Err(error) => println!("Error: {}", error),
            }
        }
    }
}
