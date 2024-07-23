

use std::fs;
use std::path::Path;


/*
use std::process;
*/
pub struct Commands;

impl Commands {
    pub fn new() -> Self {
        Commands {}
    }

    pub fn process(&self, cmd: &str) ->bool {
        // processing the command lines 
        let parts : Vec<&str> = cmd.trim().split_whitespace().collect();
        let cmd_name = parts[0];
        let args = &parts[1..];

        match cmd_name {
            "help" => self.print_help(),
            "quit" => return false,
            "clean" => self.clean(args),
            // "dup" => self.remove_duplicated_files(),
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
        println!("{} ", input);
        // a list of dirs 
        let dirs = vec!["/Library/Logs", "/var/log"];//, "~/Library/Caches" ];

        
        for dir_str in &dirs  {
            let dir = Path::new(dir_str);
            if dir.exists() && dir.is_dir() {
                println!("procssing dir : {:?}", dir);
                for f in fs::read_dir(dir).unwrap() {
                    let f = f.unwrap();
                    let path = f.path();
                    if path.is_dir() {
                        if let Err(err) =  fs::remove_dir_all(&path) {
                            println!("can not remove dir all {:?}: {}, please run it as root user", path, err);
                        }
                       
                    }else {
                        if let Err(err) = fs::remove_file(&path){
                            println!("can not remove file {:?}: {}, please run it as root user", path, err);
                        }
                    }

                }
            }
        }

    }

    
   


}