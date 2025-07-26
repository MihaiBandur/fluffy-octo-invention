use std::env;
use std::process;
use  minigrep::Config;


fn main()
{
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for {}",config.query);
    //println!("In file {}",config.file_path);


    //let contents = fs::read_to_string(config.file_path)
      //              .expect("Should have been able to read the file");

      if let  Err(e)= minigrep::run(config)
      {
        eprintln!("Application erroe: {e}");
        process::exit(1);
      }
    
   // println!("With text:\n{contents}");

    
}

/*fn parse_config(args: &[String]) -> Config
{
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config{ query, file_path }
}*/

