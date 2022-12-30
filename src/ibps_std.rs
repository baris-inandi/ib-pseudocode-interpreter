pub fn generate_stdlib() -> String {
    let mut stdlib = String::new();
    stdlib.push_str(&format!("\n{}", std::include_str!("../stdlib/stdcoll.py")));
    stdlib.push_str(&format!("\n{}", std::include_str!("../stdlib/stdqueue.py")));
    stdlib.push_str(&format!("\n{}", std::include_str!("../stdlib/stdstack.py")));
    stdlib
}
