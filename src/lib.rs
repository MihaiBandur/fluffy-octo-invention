use std::env;
use std::error::Error;
//use std::file;
use std::fs;
//use std::process;

pub struct Config {
   pub  query: String,
   pub  file_path: String,
   pub ignore_case: bool,
}

impl Config{
    /*#[warn(dead_code)]
    pub fn new(args: &[String]) -> Config{

        if args.len()<3 
        {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query: query, file_path: file_path }
    }*/

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>
    {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return  Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query: query, file_path: file_path, ignore_case: ignore_case })
    }


}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
           // .expect("Should have been able to read the file");

    let results = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    }else{
        search(&config.query, &contents)
    };

    for line in results{
        println!("{line}");
    }
   

    //println!("With text:\n{contents}");

    Ok(())


}
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_resutl(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query ="rUsT";
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