use clap::Parser;
use noa::noa::Noa;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    script: String,
}
fn main() {
    let args = Args::parse();
    let mut noa = Noa::new();
    noa.load_libray();
    if args.script.is_empty() {
        eprintln!("Script was not provided as argument");
    } else {
        match noa.run_file(args.script) {
            Ok(num) => {
                std::process::exit(num as i32);
            }
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
    }
}
