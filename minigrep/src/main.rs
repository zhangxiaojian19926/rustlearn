use std::env;
use std::process::exit;
use minigrep::run;
use minigrep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();

    // unwarp 方法是将返回的枚举进行解析, match 的返回值是 config
    // Config::new 的返回值作为match的要求
    let config = match Config::new(&args){
                        Ok(config) => config,
                        Err(err) => {
                                println!("input arg error {}", err);
                                Config {
                                    query: String::from("hello"),
                                    filename: String::from("world"),
                                }
                            }
                    };
    println!("serch for {}", config.query);

    println!("filename {}", config.filename);

    if let Err(e) = run(config) {
        println!("error is {}", e);

        exit(1);
    }
}
