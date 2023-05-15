pub mod config {

    use Opt::*;
    pub enum Opt {
        Asm, // only output asm
        Ast, // print AST
        Help, // print help
    }

    impl Opt {
        pub fn from_char(c: char) -> Option<Self> {
            match c {
                'S' => Some(Asm),
                's' => Some(Ast),
                'h' => Some(Help),
                _ => None
            }
        }
        pub fn from_str(str: &str) -> Option<Self> {
            match str {
                "ast" => Some(Ast),
                "help" => Some(Ast),
                _ => None,
            }
        }
    }

    #[derive(Default)]
    pub struct Config {
        pub output_asm: bool,
        pub print_ast: bool,
        pub print_help: bool,
    }

    impl Config {
        pub fn add_option(&mut self, o: Opt) {
            unsafe {
                match o {
                    Asm => GLOBAL_CONFIG.output_asm = true,
                    Ast => GLOBAL_CONFIG.print_ast = true,
                    Help => GLOBAL_CONFIG.print_help = true,
                }
            }
        }
    }

    pub static mut GLOBAL_CONFIG: Config = Config {
        output_asm: false,
        print_ast: false,
        print_help: false,
    };

}

pub mod logging {

    #[macro_export]
    macro_rules! log_d {
        ($x:expr $(, $y:expr)*) => {
            eprintln!(concat!("[DEBUG]: ", $x), $($y),*)
        }
    }

    #[macro_export]
    macro_rules! log_i {
        ($x:expr $(, $y:expr)*) => {
            eprintln!(concat!("[INFO]: ", $x), $($y),*)
        }
    }

    use std::process::exit;
    use crate::lexer::TokenKind;

    pub fn cmd_err(program_name: &str, message: &str) -> ! {
        eprintln!("{program_name}: {message}");
        exit(1);
    }
    pub fn io_err(e: std::io::Error) -> ! {
        eprintln!("IO error: {}", e);
        exit(1);
    }
    pub fn asm_err(text: &str) -> ! {
        eprintln!("Assembly error: {}", text);
        exit(1);
    }
    pub fn name_err(name: &str) -> ! {
        eprintln!("NameErr: `{}` not defined", name);
        exit(1);
    }
    pub fn linking_err(text: &str) -> ! {
        eprintln!("Linking error: {}", text);
        exit(1);
    }
    pub fn syntax_err(text: &str) -> ! {
        eprintln!("Syntax error: {}", text);
        exit(1);
    }
    pub fn unexpected_token(expected: TokenKind, found: TokenKind) -> ! {
        eprintln!("Unexpected token {:?}, expected {:?}", found, expected);
        exit(1);
    }
    pub fn no_expected_token(expected: TokenKind) -> ! {
        eprintln!("Expected token {:?}, but found nothing", expected);
        exit(1);
    }
}

mod lexer;
mod parser;
mod nasm;

pub fn compile_file(input_path: &str, output_path: &str) {
    let contents = std::fs::read_to_string(input_path).unwrap_or_else(|e| logging::io_err(e));
    let mut lexer = lexer::Lexer::new(contents.chars().collect());
    let (funs, data) = parser::parse(&mut lexer);
    unsafe {
        if config::GLOBAL_CONFIG.print_ast {
            log_d!("AST: {:?}", funs);
        }
    }
    nasm::to_linux_nasm_x64(output_path, &funs, &data);
}
