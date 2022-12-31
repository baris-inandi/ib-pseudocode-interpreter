pub fn generate_stdlib() -> String {
    fn to_valid(code: &str) -> String {
        return format!(
            "\n{}",
            code.lines()
                .filter(|line| !line.trim().ends_with("# stdlibignore"))
                .collect::<Vec<&str>>()
                .join("\n")
        );
    }
    let mut stdlib = String::new();
    stdlib.push_str(&to_valid(std::include_str!("../stdlib/header.py")));
    stdlib.push_str(&to_valid(std::include_str!("../stdlib/stdcoll.py")));
    stdlib.push_str(&to_valid(std::include_str!("../stdlib/stdqueue.py")));
    stdlib.push_str(&to_valid(std::include_str!("../stdlib/stdstack.py")));
    stdlib.push_str(&to_valid(std::include_str!("../stdlib/stdarr.py")));
    stdlib.push_str("\n");
    stdlib
}
