//use std::env;
use std::error::Error;
//use std::file;
use std::fs;
//use std::process;

pub struct Config {
   pub  query: String,
   pub  file_path: String,
}

impl Config{
    #[warn(dead_code)]
    pub fn new(args: &[String]) -> Config{

        if args.len()<3 
        {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query: query, file_path: file_path }
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3
        {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query: query, file_path: file_path })
    }


}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
           // .expect("Should have been able to read the file");

    for line in search(&config.query, &contents)
    {
        println!("{line}");
    }  

    //println!("With text:\n{contents}");

    Ok(())


}
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut  results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
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
}
