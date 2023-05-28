use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

// 写一个单元测试的用例
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

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

// 搜索字符串，大小写敏感
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// 搜索字符串，大小写不敏感
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// 将主函数中的功能提取到接口中
pub fn run(config: Config) -> Result<(),Box<dyn Error>> {
    // config.filename.clone() 复制一个变量传递到read_to_string中，防止config的所有权转移
    let contents: String = fs::read_to_string(config.filename.clone()).expect("read string is error");

    for line in search(&config.query, &contents){
        println!("{:?}\n", line);
    }

    Ok(())
}

// impl为结构体实现方法
// 表明Result枚举来丰富返回值
impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str>{

        if args.len() < 3 {
            println!("input args error!!! args is {}", args.len());
            return Err("error");
        }

        let query = args[1].clone();
        
        let filename = args[2].clone();

        return Ok(Config { query, filename});
    }
}