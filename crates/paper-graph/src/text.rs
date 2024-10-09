pub fn trim_brace(s: &str) -> &str {
    &s[1..s.len() - 1]
}

pub fn wrap_text(s: &str, n: usize) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        // Check if adding this word would exceed the limit
        if current_line.len() + word.len() + 1 > n {
            // If current line is not empty, push it to lines
            if !current_line.is_empty() {
                lines.push(current_line.trim().to_string());
            }
            // Start a new line with the current word
            current_line = word.to_string();
        } else {
            // If current line is empty, just add the word
            if !current_line.is_empty() {
                current_line.push(' '); // Add space before the word
            }
            current_line.push_str(word);
        }
    }

    // Add the last line if it's not empty
    if !current_line.is_empty() {
        lines.push(current_line.trim().to_string());
    }

    lines.join("\\n")
}
