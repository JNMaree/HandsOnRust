#![allow(non_snake_case)]
#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn main() {
    let mut list: Vec<Phrase> = Vec::new();
    list.push(Phrase::new("one",1));
    list.push(Phrase::new("two",2));
    list.push(Phrase::new("three",3));
    let mut pass = false;
    while !pass {
        pass = read_passphrase(&list);
    }
    println!("program completed successfully!");
}

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
        println!("pass:{}, numeric:{}", self.pass, self.numeric);
    }

    fn eq(&self, phrase:&str) -> bool{
        return self.pass.eq(phrase);
    }
}

fn read_passphrase(list: &[Phrase]) -> bool{
    let mut uinput = String::new();
    println!("passphrase:");
    stdin().read_line(&mut uinput).expect("read_failed");
    for i in list {
        if i.eq(&uinput.trim()) {
            i.print_pass();
            uinput.clear();
            return true
        }
    }
    println!("phrase:{:?} not recognised!", uinput.trim());
    uinput.clear();
    false
}
