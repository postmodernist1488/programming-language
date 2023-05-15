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

fn main() {

    let mut args = Args::construct();
    let mut input_file_path = None;
    let mut output_file_path = None;

    while let Some(arg) = args.v.next() {
        let mut chars = arg.chars();
        match chars.next() {
            Some('-') => {
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
            }
            Some(_) => input_file_path = Some(arg),
            None => (),
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
