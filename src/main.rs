use clap::Parser;
use noa::noa::Noa;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::new())]
    script: String,
}
fn main() {
    let args = Args::parse();
    let mut noa = Noa::new();
    noa.load_libray();
    if args.script.is_empty() {
        loop {
            let mut line: String = String::new();
            print!(">> ");
            io::stdout().flush().expect("Failed to flush");
            match io::stdin().read_line(&mut line) {
                Ok(_) => match noa.run(line) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                },
                Err(_) => {
                    println!("Failed to read line shutting down");
                    break;
                }
            }
        }
    } else {
        if let Err(err) = noa.run_file(args.script) {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
