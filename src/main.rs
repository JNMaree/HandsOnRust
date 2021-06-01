use std::{io::stdin};
#[allow(non_snake_case)]


fn main() {
    let commands = vec!["exit", "help", "new"];
    let mut sInput = String::new();
    
    //infinite loop for program execution
    let mut loopCondition = true;
    loop{

        println!("Enter Command:");
        stdin().read_line(&mut sInput).expect("Command:{} does not exist");

        for i in 0..commands.len(){
            if sInput.to_ascii_lowercase() == commands[i] {
                if i == 0 {
                    loopCondition = false;
                }
                println!("Command:{} executed!", &commands[i]);ef
            }
        }

        if !loopCondition {
            break;
        }
    }

    

}