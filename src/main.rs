use std::env;

use chrono::format::strftime;

#[derive(Debug)]
struct Config {
    simulate: bool,
    exchange: String,
    pairs: String,
    count: u64,
}

fn main() {

    let config = Config::new();

    println!("{:?}", config);
    // println!("In file {}", file_path);
}

impl Config{
    fn new( )->Config{
    let mut config = Config {
        simulate: true,
        exchange: String::from("binance"),
        pairs: String::from("btc/usdt"),
        count: 0,
    };

    let mut args = env::args()
    .skip(1)   // skip program name
    .peekable(); // allow looking forward one

    match args.peek().map(|x| x.as_ref()) {
        Some("--exchange") => {
            args.next();
            let val = args.next().ok_or("err").expect("");
            config.exchange = val
        }
        Some("--pairs") => {
            let val = args.next().ok_or("err").expect(""); // Skip the flag
            config.pairs =val;
        }
        _ => {
            println!("handle no option");
        }
    }
    config
    }
}
