use std::io::{Write, stdin, stdout};
use std::process::Command;
fn main() {
    loop {
        print!("> ");
        stdout().flush().expect("something went wrong with flushing stdout.");
        
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();
        let mut child = Command::new(command)
            .spawn()
            .unwrap();
        
        child.wait().expect("something went wrong with waiting for command to finish.");
    }
}
