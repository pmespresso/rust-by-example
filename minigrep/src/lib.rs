// Rust standard file system library
use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // rather than panic on error, `?` will return the error value for the caller to handle
    let contents = fs::read_to_string(config.filename)?; // returns Result<String> of file contents

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // the intended behavior is to be able to call
        // cargo run query_string filename
        // and args[0] is the program name
        // hence:
        let query = args[1].clone();   // args in main() is the owner, and this function borrows it
        let filename = args[2].clone(); // clone() makes a full copy of the args data in memory and take ownership

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive }) // if Config tried to take ownership of args without cloning, that would violate rust's borrowing rules.

        // because the args in main() which is lending it will go out of scope then (dereferenced)
        // since you can't have two pointers to the same variable in memory

        /*
        The Trade-Offs of Using clone
            There’s a tendency among many Rustaceans to avoid using clone to fix ownership problems
            because of its runtime cost. In Chapter 13, you’ll learn how to use more efficient methods
            in this type of situation. But for now, it’s okay to copy a few strings to continue making
            progress because you’ll make these copies only once and your filename and query string are very small.
            It’s better to have a working program that’s a bit inefficient than to try to hyperoptimize code on your
            first pass. As you become more experienced with Rust, it’ll be easier to start with the most efficient
            solution, but for now, it’s perfectly acceptable to call clone.
        */
    }
}

// the `a is a lifetime specifier. The return type has a borrowed value but
// it needs to know whether it is borrowed from query or from contents
// since we need to return a slice of `contents` that matches `query`,
// contents is the value we connect to the return value using lifetime syntax
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // this returns a String and assigns to query in this scope
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) { // add the amperstand as the signature of contains() is defined to take a string slice
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
