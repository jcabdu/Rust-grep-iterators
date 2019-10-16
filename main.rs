use std::env;           // to read arguments from the prompt - 
use std::process; 

use minigrep_0_3::Config; 

fn main() {
    let config= Config::new (env::args()).unwrap_or_else(|err| {        // passing ownership of the iterator to Config::new - 
        eprintln! ("Problem parsing arguments: {}", err); 
        process::exit(1); 
    }); 

    println! ("Searching for the query: {}", config.query); 
    println! ("In the file: {}", config.filename); 

    if let Err(e)= minigrep_0_3::run (config) {
        eprintln! ("Application error: {}", e); 
        process::exit(1); 
    }
}
