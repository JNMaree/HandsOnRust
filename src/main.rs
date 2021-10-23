#![allow(non_snake_case)]
#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;
//const STRING_BUFFER: usize = 256;

fn main() {
    let list = ["one","two","three"];
    let mut pass = false;
    while !pass {
        pass = read_passphrase(&list);
    }
    println!("program completed successfully!");
}

fn read_passphrase(list: &[&str]) -> bool{
    let mut uinput = String::new();
    println!("passphrase:");
    stdin().read_line(&mut uinput).expect("read_failed");
    for i in list {
        if i.eq(&uinput.trim()) {
            println!("user accepted:{:?}", i);
            uinput.clear();
            return true
        }
    }
    println!("phrase:{:?} not recognised!", uinput.trim());
    uinput.clear();
    false
}
