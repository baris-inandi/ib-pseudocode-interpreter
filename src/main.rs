use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;
pub mod remove_comments;
pub mod run;
pub mod stdlib;
pub mod to_ir;

use remove_comments::remove_comments;

fn main() {
    let filepath = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: ibps <path>");
        exit(1)
    });
    let filename = String::from(
        Path::new(&filepath)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or(""),
    );

    let contents = remove_comments(&fs::read_to_string(&filepath).unwrap_or_else(|_| {
        eprintln!("No such file or directory");
        exit(1)
    }));
    let stdlib = stdlib::generate_stdlib();
    let mut pycode = format!("{}{}", stdlib, to_ir::to_ir(contents));
    pycode = pycode
        .lines()
        .filter(|line| line.trim().len() > 0)
        .collect::<Vec<&str>>()
        .join("\n");

    // println!("{}", pycode);

    match run::run_py(&pycode, &filename) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Runtime error");
            exit(1);
        }
    };
}
