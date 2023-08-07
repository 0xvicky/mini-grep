// use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(config.filename)?;

    let result: Vec<&str>;
    if config.case_sensitive.parse().unwrap() {
        result = search_sensitive(&config.query, &content);
    } else {
        result = search_insensitive(&config.query, &content);
    };

    if result.len() == 0 {
        println!("Query not found !");
    } else {
        println!("Result\n:{:?}", result);
    }

    Ok(())
}

pub struct Config {
    //To relate both the query and the filename
    pub query: String,
    pub filename: String,
    pub case_sensitive: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 4 {
            return Err("Insufficient Arguments");
        }
        let query: &String = &args[1];
        let filename: &String = &args[2];
        let is_sensitive: &String = &args[3];

        println!("Case rule:{}", is_sensitive);
        Ok(Config {
            query: query.clone(),
            filename: filename.clone(),
            case_sensitive: is_sensitive.clone(),
        })
    }
}

pub fn search_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results
}

pub fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        println!("{}", line.to_lowercase());
        let query = query.to_lowercase();
        if line.to_lowercase().contains(&query) {
            result.push(line.trim());
        }
    }
    result
}
#[cfg(test)]

mod test {
    use super::*;

    #[test]

    fn search_test() {
        let query = "fuct";
        let content = "/
        Pick three:
        Safe, Fast, Profuctive
        Exit
        ";

        assert_eq!(
            vec!["Safe, Fast, Profuctive"],
            search_sensitive(query, content)
        );
    }

    #[test]
    fn case_insensitive_test() {
        let query = "rUsT";
        let content = "
        Pick three:
        Rust:
        Safe,  Fast, Profuctive
        Exit
        trust
        ";
        assert_eq!(vec!["Rust:", "trust"], search_insensitive(query, content));
    }
}
