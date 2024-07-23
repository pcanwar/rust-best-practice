

pub struct Commands;

impl Commands {
    pub fn new() -> Self {
        Commands {}
    }

    pub fn process(&self, cmd: &str) ->bool {
        let parts : Vec<&str> = cmd.trim().split_whitespace().collect();
        let cmd_name = parts[0];
        let args = &parts[1..];

        match cmd_name {
            "help" => self.print_help(),
            "quit" => return false,
            "clean" => self.clean(args),
            _ => println!("{} is unknown command", cmd_name),
        }
        true
    }

    fn print_help(&self) {
        println!(" 
            help - Print this help info
            quit - Quit this app
            clean - Cleaning Files");
    }

    fn clean(&self,  _args: &[&str]) {
        // clean dirs: 
        // /Library/Logs, 
        // ~/Library/Caches,
        //  /var/log
        let input = _args.join(" ");
        println!("{} ", input)
    }

    // write a fn to remove duplicate files in a directory
}