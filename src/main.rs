#![allow(non_snake_case)]
#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

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

#[derive(Debug)]
struct Phrase{
    pass: String,
    numeric: i8
}

impl Phrase{
    fn new(str:&str, num:i8) -> Self {
        Self{
            pass:str.to_lowercase(),
            numeric:num
        }
    }
    fn print_pass(&self) {
        println!("PASSED:{:#?}", self);
    }
}

fn read_passphrase(list: &mut Vec<Phrase>) -> i8 {
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
            let num: i8 = uinput.trim().parse().unwrap();
            list.push(Phrase::new(&newpass, num));
            1
        } else {
            println!("Pass:{} NOT added!", newpass);
            uinput.clear();
            0
        }
    }
}
