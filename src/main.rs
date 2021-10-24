#![allow(non_snake_case)]
#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;
use crate::PhraseDivision::{*};

#[derive(Debug)]
enum PhraseDivision{
    NotDefined,
    Prime,
    Odd,
    Even,
}

#[derive(Debug)]
struct Phrase{
    pass: String,
    numeric: u8,
    division: PhraseDivision,
}

fn main() {
    let mut list: Vec<Phrase> = vec![
        Phrase::new("one",1),
        Phrase::new("two",2),
        Phrase::new("three",3),
    ];
    loop {
        if read_passphrase(&mut list) > 1 {
            break;
        }
    }
    println!("program completed successfully!");
}

fn is_prime(num: u8) -> bool {
    if num == 1 {
        false
    } else if num == 2 {
        true
    } else {
        let mut factor = 2;
        let root:u8 = (num as f32).sqrt() as u8;
        while factor < root {
            if num % factor == 0 {
                return false
            }
            factor += 1
        }
        true
    }
}

impl Phrase{
    fn new(str:&str, num:u8) -> Self {
        let mut pd: PhraseDivision;
        if num % 2 == 0 {
            pd = Even;
        } else if is_prime(num) {
            pd = Prime;
        } else {
            pd = Odd;
        }
        Self{
            pass: str.to_lowercase(),
            numeric: num,
            division: pd,
        }
    }
    fn print_pass(&self) {
        println!("PASSED:{:#?}", self);
    }
}

fn read_passphrase(list: &mut Vec<Phrase>) -> u8 {
    let mut uinput = String::new();
    println!("passphrase:");
    stdin().read_line(&mut uinput).expect("read_line failed!");
    let passed = list.iter().find(|phrase| phrase.pass == uinput.trim());

    if let Some(phrase) = passed {
        // If a phrase was found in list
        phrase.print_pass();
        uinput.clear();
        2
    } else {
        // If No phrase was found
        let newpass = String::from(uinput.trim());
        println!("phrase:{:?} not recognised! Add passphrase?", newpass);
        uinput.clear();
        stdin().read_line(&mut uinput).expect("read_line failed!");
        if uinput.trim().eq_ignore_ascii_case("y"){
            uinput.clear();
            println!("Insert Numeric Equivalent:");
            stdin().read_line(&mut uinput).expect("unknown parameter inserted");
            let num: u8 = uinput.trim().parse().unwrap();
            list.push(Phrase::new(&newpass, num));
            1
        } else {
            println!("Pass:{} NOT added!", newpass);
            uinput.clear();
            0
        }
    }
}
