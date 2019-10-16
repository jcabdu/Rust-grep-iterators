use std::error::Error;              // Error trait -
use std::fs;                        // read files - 
use std::env;                       // env module to work with environment variables - 

pub struct Config {
    pub query: String, 
    pub filename: String, 
    pub case_sensitive: bool,           // adding another field for the environment variable - 
}

impl Config {
    pub fn new (mut args: std::env::Args) -> Result <Config, &'static str> {             // new associated funtion -
    // The env::args function returns an iterator of type std::env::Args - mut args by iterating over it - 

        args.next ();       // since std::env::Args implements the Iterator trait, call the next method - 
        // The first value returned by env::args is the name of the program - 
        
        let query= match args.next() {
            Some (arg) => arg, 
            None => return Err("No query string was found in the inputs"),
        }; 

        let filename= match args.next() {
            Some (arg) => arg, 
            None => return Err("No file name was found in the inputs"),
        };  

        let case_sensitive= env::var("CASE_INSENSITIVE").is_err();              
        // checking for an environment variable named CASE_INSENSITIVE - env::var fn returns a Result (Ok contains the value of the environment variable 
        // when this one is set - Err variant when the environment variable is not set) - 

// is_err method -> 
//  - if Err (env var not set) -> True -> case-sensitive search - 
//  - if CASE_INSENSITIVE set to any value -> env::var returns Ok -> is_err: False -> case-insensitive search -     
// In this case, only care if the environment variable is set or unset, not about its value (so no need to use other methods like unwrap or expect) -       

        Ok (Config {query, filename, case_sensitive}) 
    }
}

pub fn run (config: Config) -> Result <(), Box <dyn Error>> {           
    // Box <dyn(dynamic) Error> is a trait object that means the fn will return a type that implements the Error trait -
    let contents= fs::read_to_string(config.filename)?; 

    println! ("The file contains: \n{}", contents); 

    let results= if config.case_sensitive {
        search (&config.query, &contents)
    } else {
        search_case_insensitive (&config.query, &contents)
    }; 

    println! ("Results:"); 
    for line in results {
        println! ("{}", line);              // print each element (line) of the vector results - 
    }

    Ok (())
}


// TEST-DRIVEN DEVELOPMENT (TDD) PROCESS: 
//  1. Write a test that fails and run it to make sure it fails for the reason you expect.
//  2. Write or modify just enough code to make the new test pass.
//  3. Refactor the code you just added or changed and make sure the tests continue to pass.
//  4. Repeat from step 1! - 

// TDD for the "search" function that does the searching for the query string in the file contents and produce a list of lines that match the query: 

#[cfg(test)]
mod tests {
    use super::*; 

    // 1. Creating a failing test for the search function: 
    #[test]
    fn case_sensitive() {
        let query= "duct"; 
        let contents= "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";         // shouldn't get returned since the fn search is case sensitive -  

        assert_eq! (
            vec! ["safe, fast, productive."],       // returns the whole line - 
            search (query, contents)
        ); 
    }

// (**) ENVIRONMENT VARIABLES to control search done case-insensitive 
//  -> add new fn search_case_insensitive that will be called when the environment variables is SET (on): 

    #[test]
    fn case_insensitive() {
        let query= "rUsT"; 
        let contents= "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq! (
            vec! ["Rust:", "Trust me."], 
            search_case_insensitive (query, contents)
        ); 
    }

}


// Defining the search function: 
// Functional programming prefers to minimize  the amount of mutable state to make code clearer - 
//  removing the mutable state might enable a future enhancement to make searching happen in parallel - 
pub fn search <'a> (query: &str, contents: &'a str) -> Vec <&'a str> {
    // 2. Writing code to pass the test - the program has to follow these steps: 
    //  - a) Iterate through each line of the contents.
    //  - b) Check whether the line contains our query string.
    //  - c) If it does, add it to the list of values we’re returning.
    //  - If it doesn’t, do nothing.
    //  - Return the list of results that match.

    
    // a) lines method (for line-by-line iteration of strings) -> returns an iterator: 
    contents.lines()        // use iterator adaptors: 
        .filter(|line| line.contains(&query))       // keep only the lines that line.contains(query) returns true for - 
        .collect()
}

// (**) Implementing the search_case_insensitive fn - (lowercase the query and the line before comparing them) : 

pub fn search_case_insensitive <'a> (query: &str, contents: &'a str) -> Vec <&'a str> {
    let query= query.to_lowercase();            // query is a shadowed variable - of type String (rather than a string slice) -            
    
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
