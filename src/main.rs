use std::{env, process};

mod error;
mod threshold;

fn usage() {
    println!(
        "USAGE:
  {} <COMMAND> [ARGS...]

COMMANDS:
  threshold <START> <STOP>     Set start and stop charge thresholds
  fullcharge                   Set threshold to enable immediate charging until full

OPTIONS:
  -h, --help       Print command-specific usage
  -V, --version    Print version information",
        env!("CARGO_PKG_NAME"),
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
                eprintln!("Error: missing start/stop thresholds\n");
                usage();
                process::exit(0x0101);
            }

            let start: u8 = match &args[2].parse() {
                Ok(s) => *s,
                Err(_) => {
                    eprintln!("{}", error::ERR_INVALID_THRESHOLD);
                    process::exit(0x0101);
                }
            };
            let stop: u8 = match &args[3].parse() {
                Ok(s) => *s,
                Err(_) => {
                    eprintln!("{}", error::ERR_INVALID_THRESHOLD);
                    process::exit(0x0101);
                }
            };

            match threshold::set(start, stop) {
                Ok(_) => return,
                Err(e) => {
                    eprintln!("Error setting charge threshold: {}", e);
                    process::exit(0x0101);
                }
            }
        }
        "fullcharge" => match threshold::set(0, 100) {
            Ok(_) => return,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(0x0101);
            }
        },
        "-h" | "--help" => {
            println!("{}\n", env!("CARGO_PKG_DESCRIPTION"));
            usage();
        }
        "-v" | "--version" => println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        _ => {
            usage();
            process::exit(0x007F);
        }
    }
}
