#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command as ProcessCommand;

struct Commands{
    echo : String,
    exit : String,
    types : String,
}   

impl Commands {
    fn built_in(&self, command : &str)-> bool {
        command == self.echo || command == self.exit || command == self.types
    }

    fn check(&self, command: &str) {
        if self.built_in(command){
            println!("{command} is a shell builtin");
        }else {
            match ProcessCommand::new("which").arg(command).output(){
                Ok(output) if output.status.success() => {
                    let path = String::from_utf8_lossy(&output.stdout);
                    print!("{command} is {path}") //here path already ends with a newline
                }
                _ => println!("{command}: not found")
            }
        }
    }
    fn echo(&self, message: &str){
        println!("{message}");
    }
    
}

fn main() {
    let command = Commands{
        echo : String::from("echo"),
        exit : String::from("exit"),
        types : String::from("type"),
    };
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); //here readline includes a new line also
        let input = input.trim(); //<-removing new line
        
        if let Some(rest) = input.strip_prefix("type ") {
            command.check(rest);
        } 
        else if let Some(rest) = input.strip_prefix("echo ") {
            command.echo(rest);
        } else if input == "exit 0"{
            break;
        }else{
            println!("{input}: command not found ");
        }
    }
}
