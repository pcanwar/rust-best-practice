
use std::collections::HashMap;
use std::io::{self, Write};

struct Account {
    balance: HashMap<String, u32>,
}
impl Account {
    fn new() -> Self {
        Self {balance: HashMap::new()}
    }

    fn get_balance(&self, account: &str) -> u32 {
        let bal = self.balance.get(account).cloned().unwrap_or(0);
        // println!("Balance of {account} is {bal}");
        bal
    }

    fn receive_tx(&mut self, to: &str, amount:u32) {
        println!("receive {amount} tokens to {to}\n");
        *self.balance.entry(to.to_string()).or_insert(0) += amount;
    }

    fn send_tx(&mut self, from: &str, to: &str, amount:u32){
        let sender = self.balance.entry(from.to_string()).or_insert(0);
        if *sender >= amount {
            *sender -= amount;
            *self.balance.entry(to.to_string()).or_insert(0) += amount;
            println!("send {amount} to {to}");
        } else {
            println!("you have no enough tokens in your balance account.\n")
        }
    }
}

fn process_command(account: &mut Account, command: &str){
    let args: Vec<&str> = command.trim().split_whitespace().collect();
    match args.get(0){
        Some(&"se") => {
            if args.len() < 4 {
                println!("Usages send <from> <to> <amount>");
                return;
            }
            let from = args[1];
            let to = args[2];
            let amount: u32 = args[3].parse().unwrap_or(0);
            account.send_tx(from, to , amount);
        },
        Some(&"re") => {
            if args.len() < 3 {
                println!("Usages receive <to_account> <amount>");
                return;
            }
            let to = args[1];
            let amount : u32 = args[2].parse().unwrap_or(0);
            account.receive_tx(to, amount);
        },
        Some(&"bal") => {
            if args.len() < 2 {
                println!("Usage : balance <account>");
                return;
            }
            let account_name = args[1];
            let bal = account.get_balance(account_name);
            println!("Balance of {account_name} is {bal}\n")
        },
        Some(&"h") => {
            println!("Try these commands:");
            println!("  'se'   : send transaction");
            println!("\t Usage: se <from_account> <to_account> <amount>\n");
            println!("  're'   : receive a transaction");
            println!("\t Usage: re <to_account> <amount>\n");
            println!("  'bal'  : check the balance of an account");
            println!("\t Usage: bal <account_name>\n");
            println!("  'quit' : exit the program\n");
            println!("  'h'    : show help message\n");
        },
        None => println!("No command, try again"),
        _ => println!("Invalid command")
    }
 
}


fn main() {

    let mut account = Account::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("unable to read the line");

        if command.trim().eq("quit") {
            println!("Goodbye... \n");
            break;
        } else {
            process_command(&mut account, &command);
        }
    }

}
