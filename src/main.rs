use std::{env, process};

fn usage() {
    println!(
        "{}

USAGE:
  {} <COMMAND>

COMMANDS:
  threshold        Set start and stop charge thresholds
  fullcharge       Set threshold to enable immediate charging until full

OPTIONS:
  -h, --help     Print command-specific usage
  -V, --version  Print version information",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_NAME")
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        process::exit(0x0101);
    }

    let cmd = &args[1];
    match &cmd[..] {
        "threshold" => {
            if args.len() != 4 {
                eprintln!("not enough args");
                process::exit(0x0101);
            }

            let start: u8 = match &args[2].parse() {
                Ok(s) => *s,
                Err(_) => {
                    eprintln!("Error: start threshold must be between 0-100");
                    process::exit(0x0101);
                }
            };
            let stop: u8 = match &args[3].parse() {
                Ok(s) => *s,
                Err(_) => {
                    eprintln!("Error: stop threshold must be between 0-100");
                    process::exit(0x0101);
                }
            };

            threshold(start, stop);
        }
        "fullcharge" => threshold(0, 100),
        "-h" | "--help" => println!("help"),
        "-v" | "--version" => println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        _ => {
            usage();
            process::exit(0x007F);
        }
    }
}

fn threshold(start: u8, stop: u8) {
    if start > 100 || stop > 100 {
        eprintln!("Error: start and stop thresholds must be between 0-100");
        process::exit(0x0101);
    }

    if start >= stop {
        eprintln!("Error: start threshold must be less than stop threshold");
        process::exit(0x0101);
    }

    println!(
        "Battery will start charging below {}% and stop charing at {}%",
        start, stop
    )
}
