#![allow(non_snake_case)]

use std::io;

//const STRING_BUFFER: usize = 256;

fn main() {
    println!("start_program:");
    REPL();
    println!("end_program.");
}

fn REPL() -> u8
{
    loop {
        print!("Enter Command:");
        let mut uinput = String::new();
        std::io::stdin().read_line(&mut uinput).expect("Failed to read input.");
        
        if uinput.to_lowercase().eq("exit") {
            println!("exiting...");
            break;
        } else {
            println!("cmd:{} exe!", uinput.to_uppercase());
        }
    }
    return 0;
}