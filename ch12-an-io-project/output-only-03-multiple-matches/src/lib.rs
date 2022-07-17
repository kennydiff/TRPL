use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// ANCHOR: here
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
// ANCHOR_END: here

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {  // K_22716 ❓这里为何contents要生命周期，query不需要?
// pub fn search(query: &str, contents: &str) -> Vec<&str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);  // K_22716 这里影响了生命周期,所以contents得要加上生命周期声明
        }
    }
    results  // K_22716 results里面是contents的引用，所以contents的生命周期要比results长 ，所以要在contents加上生命周期声明
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
