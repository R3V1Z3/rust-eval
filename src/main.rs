extern crate eval;

use std::env;
use std::string;
use eval::{eval};

fn main() {
    for arg in env::args() {
        println!("{:?}", eval(&arg.to_string()));
    }
}
