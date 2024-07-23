
mod commands;
use commands::Commands;
use std::io::{self, Write};

fn main() {

    let cmd = Commands::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin().read_line(&mut command).expect("Fail to read the line");

        if !cmd.process(command.trim()) {
            break
        }
    }

    // let running = true;
    // while running {

    // }
    println!("GoodBye! ")


}
