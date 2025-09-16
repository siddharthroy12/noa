use clap::Parser;
use rlox::lox::Lox;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::new())]
    script: String,
}

fn main() {
    let args = Args::parse();
    let lox = Lox::new();
    if args.script.is_empty() {
        loop {
            let mut line: String = String::new();
            print!(">> ");
            io::stdout().flush().expect("Failed to flush");
            match io::stdin().read_line(&mut line) {
                Ok(v) => match lox.run(line) {
                    Ok(res) => {
                        print!("{}", res);
                    }
                    Err(e) => {
                        print!("{}", e);
                    }
                },
                Err(e) => {
                    println!("Failed to read line shutting down");
                    break;
                }
            }
        }
    } else {
        if let Err(e) = lox.run_file(args.script) {
            std::process::exit(1);
        }
    }
}
