use cli_app::Config;
use std::env;
use std::process;
fn main() {
    println!("Mini-Grep"); //Search string in a given file

    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("Error occured:{}", err);
        process::exit(1);
    });

    println!("Query: {:?}", config.query);
    // println!("Filename: {:?}", &args[2]);
    if let Err(e) = cli_app::run(config) {
        println!("Application Error:{}", e);
        process::exit(1);
    }
}
