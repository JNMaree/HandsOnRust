#![allow(non_snake_case)]

mod parser;

//const STRING_BUFFER: usize = 256;

fn main() {
    println!("start_program:");
    
    //Program Logic Start
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => usage(),
        2 => parse_input_file(),
    }

    println!("end_program.");
}
