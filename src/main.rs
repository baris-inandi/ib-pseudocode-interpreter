use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;
pub mod remove_comments;
pub mod stdlib;
pub mod to_ir;

use pyo3::prelude::*;
use remove_comments::remove_comments;

fn main() -> PyResult<()> {
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

    let contents = remove_comments(&fs::read_to_string(&filepath).expect("cannot read file"));
    let header = stdlib::generate_stdlib();
    let pycode = format!(
        r#"
def run(*args, **kwargs):
{}
"#,
        format!("{}\n{}", header, to_ir::to_ir(contents))
            .lines()
            .map(|line| format!("    {}", line))
            .collect::<Vec<String>>()
            .join("\n")
    );

    // println!("{pycode}");

    Python::with_gil(|py| -> Result<(), PyErr> {
        let fun: Py<PyAny> = PyModule::from_code(py, &pycode, &filepath, &filename)?
            .getattr("run")?
            .into();

        fun.call0(py).unwrap_or_else(|e: PyErr| {
            e.print_and_set_sys_last_vars(py);
            exit(1)
        });
        Ok(())
    })
}
