
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
        match self.balance.get(account){
            Some(&bal) => bal,
            None => 0,
        }
        
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

    println!("Procssing your command {}", command.trim());
    if command.trim().eq("se"){
        account.send_tx("A", "B", 10);
    } else if command.trim().eq("re"){
        account.receive_tx("A", 20);
    } else if command.trim().eq("bal") {
        let balance_a = account.get_balance("A");
        let balance_b = account.get_balance("B");
        println!("Balance of A: {balance_a}");
        println!("Balance of B: {balance_b}");
    }else {
        println!("Invalid command {}", command.trim());
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
