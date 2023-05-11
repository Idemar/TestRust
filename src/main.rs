#![allow(unused)]

use std::arch::x86_64::_MM_EXCEPT_INEXACT;
use std::{io, error};
use std::io::{BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name:");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Did not recive input");

    println!("Hello {}! {}", name.trim_end(), greeting);

}
