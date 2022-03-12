use std::env;
use std::process;

use text_scanner::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config=Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Issue parsing arguments: {}",err);
        process::exit(1);
    });
    
    eprintln!("Searching for {}",config.query);
    eprintln!("in file {}",config.filename);

    if let Err(e)=text_scanner::run(config) {
        eprintln!("App Error: {}",e);
        process::exit(1);
    }
    
}

