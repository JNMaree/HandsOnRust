#![allow(non_snake_case)]
#![warn(clippy::all, clippy::pedantic)]

//const STRING_BUFFER: usize = 256;

fn main() {
    let list = ["one","two","three"];
    for i in &list{
        println!("list:{}", i);
    }

}
