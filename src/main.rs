#[warn(unused_imports)]
use std::io::{self, stdout, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); //here readline includes a new line also
        let input = input.trim(); //<-removing new line
  
        if input == "exit 0"{
            break
        }

        if let Some(rest) = input.strip_prefix("echo "){ // method reutrns the string after the prefix
            writeln!(io::stdout(), "{rest}").unwrap();
        }else{
            writeln!(io::stdout(), "{input}: command not found").unwrap();
        }
    }
}
