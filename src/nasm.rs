use super::*;
use parser::{Expr, Function, Intrinsic, Data};
use std::{io::{BufWriter, Write}, process::exit, fs::read_to_string};

fn writeln(writer: &mut BufWriter<std::fs::File>, s: &[u8]) {
    writer.write(s).unwrap_or_else(|e| logging::io_err(e));
    writer.write(b"\n").unwrap_or_else(|e| logging::io_err(e));
}

fn expr_to_nasm(writer: &mut BufWriter<std::fs::File>, funs: &Vec<Function>, expr: &Expr) {

    match expr {
        Expr::Funcall {name, args} => {
            if let Some(fun) = funs.iter().find(|x| x.name == *name) {
                for arg in args {
                    expr_to_nasm(writer, funs, arg);
                }
                writeln(writer, format!("    call {}\n", fun.name).as_bytes());
            } else {
                logging::name_err(&name);
            }
        }
        Expr::StrLit{n, len} => {
            writeln(writer, format!("    push {}", len).as_bytes());
            writeln(writer, format!("    push str_{}", n).as_bytes());
        }
        Expr::Var(name) => {
            writeln(writer, format!("    mov eax, [global_var_{}]", name).as_bytes());
            writeln(writer, b"    push rax");
        }
        Expr::Intrinsic(Intrinsic::Print) => {
            // print a string on the stack
            writeln(writer, b"    mov rax, 1");
            writeln(writer, b"    mov rdi, 1");
            writeln(writer, b"    pop r8");
            writeln(writer, b"    pop rsi");
            writeln(writer, b"    pop rdx");
            writeln(writer, b"    syscall");
            writeln(writer, b"    push r8");
        }
        Expr::Intrinsic(Intrinsic::PrintNum) => {
            let text = read_to_string("src/print_num.asm").unwrap_or_else(|e| logging::io_err(e));
            writeln(writer, text.as_bytes());
        }
    }
}

fn fundef_to_nasm(writer: &mut BufWriter<std::fs::File>, fun: &Function, funs: &Vec<Function>) {
    writeln(writer, format!("{}:", fun.name).as_bytes());
    /*
    for arg in args {
        match arg {
            Expr::StrLit {n, len} => {
                writeln(writer, format!("    push {}", len).as_bytes())?;
                writeln(writer, format!("    push str_{}", n).as_bytes())?;
            }
            Expr::Funcall { name, args } => {
                todo!("function call as argument not implemented")
            }
            Expr::Intrinsic(_) => {
                panic!("Intrinsics can't be used as function arguments")
            }
        }
    }
    */

    for expr in fun.exprs.iter() {
        expr_to_nasm(writer, funs, expr);
    }

    writeln(writer, b"    ret");
}


pub fn to_linux_nasm_x64(output_filepath: &str, funs: &Vec<Function>, data: &Data) {
    use std::process::Command;
    let asm_filename: String = format!("{}.asm", output_filepath);
    let f = std::fs::File::create(&asm_filename).unwrap_or_else(|e| logging::io_err(e));
    let mut writer = std::io::BufWriter::new(f);
    writeln(&mut writer, b"BITS 64");
    writeln(&mut writer, b"global _start");
    writeln(&mut writer, b"section .text");
    writeln(&mut writer, b"_start:");
    writeln(&mut writer, b"    call main");
    writeln(&mut writer, b"exit:");
    writeln(&mut writer, b"    mov rdi, 0");
    writeln(&mut writer, b"    mov rax, 60");
    writeln(&mut writer, b"    syscall");

    for fun in funs {
        fundef_to_nasm(&mut writer, fun, funs);
    }

    let mut bytes_repr = String::new();
    for (n, s) in data.strings.iter().enumerate() {

        for byte in s.as_bytes() {
            bytes_repr.push_str(&byte.to_string());
            bytes_repr.push(',');
        }

        writeln(&mut writer, format!("str_{n} db {bytes_repr}").as_bytes());
        bytes_repr.clear();
    }

    for var in data.globals.iter() {
        writeln(&mut writer, format!("global_var_{} dd 0", var).as_bytes());
    }
    writeln(&mut writer, b"section .bss");
    writeln(&mut writer, b"__stringspace resb 10");

    drop(writer);

    unsafe {
        if config::GLOBAL_CONFIG.output_asm {
            exit(0);
        }
    }

    fn cleanup(files: &Vec<String>) {
        for file in files {
            std::fs::remove_file(file).unwrap_or(());
        }
    }

    let mut produced_files = vec![asm_filename.clone()];

    log_i!("Generating out.o with nasm...");
    let nasm_status = Command::new("nasm").args(["-g", "-felf64", &asm_filename, "-o", "out.o"]).status()
        .unwrap_or_else(|e| logging::asm_err(&format!("failed to run nasm: {}", e)));
    if nasm_status.success() {
        produced_files.push("out.o".to_string());
        log_i!("Linking out.o with ld...");
        let ld_status = Command::new("ld").args(["-o", output_filepath, "out.o"]).status()
        .unwrap_or_else(|e| logging::linking_err(&format!("failed to run ld: {}", e)));
        cleanup(&produced_files);
        if !ld_status.success() {
            logging::linking_err("ld failed")
        }
    } else {
        cleanup(&produced_files);
        logging::asm_err("nasm failed");
    }
}

