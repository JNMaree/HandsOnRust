use std::string;



fn main() {
    let x = "Hello";
    let y = "World";

    let mut z = x.to_owned();
    z.push_str(y);

    println!("Z:{}", z);
}

