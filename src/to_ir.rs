use regex::Regex;
mod regex_replace;

pub fn to_ir(code: String) -> String {
    let code_new = regex_replace::regex_replace(&code);
    let mut out = String::new();
    for line in code_new.lines() {
        let l = line.trim();
        if l.starts_with("output") {
            let spaces = line.split("output").next().unwrap();
            let args = line.split("output").nth(1).unwrap_or("").trim();
            out.push_str(&format!("{}print({})\n", spaces, args));
        } else if l.starts_with("input") {
            let spaces = line.split("input").next().unwrap();
            let args = line.split("input").nth(1).unwrap_or("").trim();
            out.push_str(&format!("{}input({})\n", spaces, args));
        } else if l.starts_with("loop while") {
            let spaces = line.split("loop while").next().unwrap();
            let args = line.split("loop while").nth(1).unwrap_or("").trim();
            out.push_str(&format!("{}while {}:\n", spaces, args));
        } else if l.starts_with("loop until") {
            let spaces = line.split("loop until").next().unwrap();
            let args = line.split("loop until").nth(1).unwrap_or("").trim();
            out.push_str(&format!("{}__until_flag = True\n", spaces));
            out.push_str(&format!("{}while __until_flag or {}:\n", spaces, args));
            out.push_str(&format!("{}{}__until_flag = False\n", spaces, spaces));
        } else if l.starts_with("loop") {
            // for loop
            let spaces = line.split("loop").next().unwrap();
            let re = Regex::new(r"loop \w from \s*.*\s* to \s*.*\s*$").unwrap();
            let is_range = re.is_match(l);
            if is_range {
                // Split the input string into tokens
                let tokens: Vec<&str> = l.split_whitespace().collect();

                // Check that the input string has the correct format
                if tokens[0] != "loop" || tokens[2] != "from" || tokens[4] != "to" {
                    panic!("Invalid input string");
                }

                // Extract the loop variable and the loop bounds
                let loop_variable = tokens[1];
                let start = tokens[3];
                let end = tokens[5];

                // Build the output string
                out.push_str(&format!(
                    "{}for {} in range({}, {} + 1):\n",
                    spaces, loop_variable, start, end
                ));
                continue;
            }
            let args = line.split("loop").nth(1).unwrap_or("").trim();
            out.push_str(&format!("{}for {}:\n", spaces, args));
        } else if l.starts_with("if") {
            // turn "if condition then" to "if condition:"
            let mut tokens: Vec<&str> = l.split_whitespace().collect();
            let spaces = line.split("if").next().unwrap();
            if *tokens.get(tokens.len() - 1).unwrap_or(&"") == "then" {
                tokens.pop();
            }
            out.push_str(&format!("{}{}:\n", spaces, tokens.join(" ")))
        } else if l.starts_with("else if") {
            let mut tokens: Vec<&str> = l.split_whitespace().collect();
            tokens.remove(0);
            tokens.remove(0);
            tokens.insert(0, "elif");
            if *tokens.get(tokens.len() - 1).unwrap_or(&"") == "then" {
                tokens.pop();
            }
            out.push_str(&format!(
                "{}{}:\n",
                line.split("else if").next().unwrap(),
                tokens.join(" ")
            ));
        } else if l.starts_with("else") {
            let mut tokens: Vec<&str> = l.split_whitespace().collect();
            if *tokens.get(tokens.len() - 1).unwrap_or(&"") == "then" {
                tokens.pop();
            }
            out.push_str(&format!(
                "{}{}:\n",
                line.split("else").next().unwrap(),
                tokens.join(" ")
            ));
        } else if l.starts_with("sub") {
            let args = l.split("sub").nth(1).unwrap_or("").trim();
            let spaces = line.split("sub").next().unwrap();
            out.push_str(&format!("{}def {}:\n", spaces, args));
        } else if l.starts_with("function") {
            let args = l.split("function").nth(1).unwrap_or("").trim();
            let spaces = line.split("function").next().unwrap();
            out.push_str(&format!("{}def {}:\n", spaces, args));
        } else if l.starts_with("end ") {
            out.push_str("\n");
            continue;
        } else {
            out.push_str(&format!("{}\n", line));
        }
    }
    return out;
}