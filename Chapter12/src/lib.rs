use std::fs::File;
use std::io::prelude::*; // glob import -> Bring everything that is available in `...::prelude`
use std::error::Error;

pub struct Config {
    query : String,
    filename : String,
    kase : String,
}

impl Config {
    pub fn parse(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");

        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let kase = args[3].clone();
        if kase.is_empty() {
            let kase = true;
        }
        Ok(Config { query, filename, kase })
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", conf.query);
    println!("In file {}", conf.filename);

    let mut file = File::open(conf.filename)?;
    let mut con = String::new();

    file.read_to_string(&mut con)?;   // Same as:
    // Read::read_to_string(&mut file, &mut con).expect("error reading file");

    let r = if conf.kase.to_lowercase() == "false" {
        search_insenstive(&conf.query, &con)
        } else {
        search_senstive(&conf.query, &con)
    };

    for line in r {
        println!("{}", line);
    }


    Ok(())
}

// Only contents get LA but query doesn't as query is read-only datatype and
// content is the variable where data is extracted and returned.
pub fn search_senstive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut a = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            a.push(line);
        }
    }
    a
}

pub fn search_insenstive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut a = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(query.to_lowercase().as_str()) { //converts string to &str
            a.push(line);
        }
    }
    a
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        let query = "dUCT";
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three";
        assert_eq!(vec!["        safe, fast, productive."], search_insenstive(query, content));
    }
}