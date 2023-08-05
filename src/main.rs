use std::env;
use std::fs;
use std::process;

fn main() {
    println!("Mini-Grep"); //Search string in a given file

    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error occured:{}", err);
        process::exit(1);
    });

    println!("Query: {:?}", config.query);
    // println!("Filename: {:?}", &args[2]);
    let content =
        fs::read_to_string(config.filename).expect("Something went wrong while reading the file");
    println!("The File Content:{:?}", content);
}

struct Config {
    //To relate both the query and the filename
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Insufficient Arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        Ok(Config {
            query: query.clone(),
            filename: filename.clone(),
        })
    }
}
