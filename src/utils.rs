pub fn collect_text(parent: &scraper::ElementRef, min_length: usize, collapse_brackets: bool, language_separator_text: Option<&str>) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = vec![];
    let mut current = vec![];
    
    for t in parent.text() {
        let trimmed = t.trim();
        if Some(trimmed) == language_separator_text {
            out.push(current);
            current = vec![];
        }
        else {
            if collapse_brackets && trimmed.starts_with("(") && trimmed.ends_with(")") && out.len() > 0 {
                let mut last = current.pop().unwrap();
                last = format!("{} {}", last, trimmed);
                current.push(last);
            }
            else {
                if trimmed.len() >= min_length {
                    current.push(trimmed.to_string());
                }
            }
        }
    }
    out.push(current);

    out
}