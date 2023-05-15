use std::process::exit;

use programming_language::compile_file;
use programming_language::logging::cmd_err;
use programming_language::config;

#[derive(Debug)]
struct Args {
    program_name: String,
    v: std::env::Args
}

//splits commandline arguments into program name and args
impl Args {
    fn construct() -> Self {
        let mut iterator = std::env::args();
        let program_name = iterator.next().expect("Program name is always the first argument");

        Args {
            program_name,
            v: iterator
        }
    }
}

fn help() {
    println!("Options:");
    println!("    -h | --help - print this help message");
    println!("    -o <file>   - supply output filename");
    println!("    -S          - output asm");
    println!("    -s | --ast  - print AST (debug)");
}

fn main() {

    let mut args = Args::construct();
    let mut input_file_path = None;
    let mut output_file_path = None;

    while let Some(arg) = args.v.next() {

        if arg.starts_with("--") {
            if let Some(opt) = config::Opt::from_str(&arg[2..]) {
                unsafe {
                    config::GLOBAL_CONFIG.add_option(opt);
                }
            } else {
                cmd_err(&args.program_name, &format!("unknown flag `{}`", arg));
            }
        } else if arg.starts_with('-') {
            let chars = arg[1..].chars();
            for flag in chars {
                match flag {
                    'o' => {
                        output_file_path = args.v.next();
                        if output_file_path.is_none() {
                            cmd_err(&args.program_name, "no output file provided for `-o` option");
                        }
                    }
                    other => {
                        if let Some(opt) = config::Opt::from_char(other) {
                            unsafe {
                                config::GLOBAL_CONFIG.add_option(opt);
                            }
                        } else {
                            cmd_err(&args.program_name, &format!("unknown flag `{}`", other));
                        }
                    }
                }
            }
        } else {
            input_file_path = Some(arg)
        }
    }

    unsafe {
        if config::GLOBAL_CONFIG.print_help {
            help();
            exit(0);
        }
    }

    if input_file_path.is_none() {
        cmd_err(&args.program_name, "file for compilation hasn't been provided");
    }
    if output_file_path.is_none() {
        let stem = std::path::Path::new(input_file_path.as_ref().unwrap()).file_stem()
            .unwrap_or_else(|| cmd_err(&args.program_name, &format!("input filepath is empty")));

        output_file_path = Some(stem.to_str().unwrap().to_owned());
    }
    
    compile_file(&input_file_path.unwrap(), &output_file_path.unwrap());
}
