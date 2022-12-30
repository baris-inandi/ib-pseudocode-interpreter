use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;
pub mod to_ir;

use pyo3::prelude::*;

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

    let contents = fs::read_to_string(&filepath).expect("cannot read file");

    let header = String::from(
        r"from queue import LifoQueue as PyQueue
from collections import deque as PyStack
class Queue(PyQueue):
    def __init__(self):
        super().__init__()
    def dequeue(self):
        return super().get()
    def enqueue(self, item):
        super().put(item)
    def __str__(self):
        return 'Queue' + str(list(self.queue))
class Stack(PyStack):
    def __init__(self):
        super().__init__()
    def pop(self):
        return super().pop()
    def push(self, item):
        super().append(item)
    def __str__(self):
        return 'Stack' + str(list(self))
class Collection:
    def __init__(self):
        self.inner = []
output = print
null = None
nil = None
none = None
true = True
false = False
",
    );

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

    Python::with_gil(|py| {
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
