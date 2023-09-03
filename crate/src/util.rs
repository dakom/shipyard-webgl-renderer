pub fn insert_at_first_blank_line(src: &str, dest: &str) -> String {
    if let Some(pos) = dest.find("\n") {
        let (start, end) = dest.split_at(pos);
        return format!("{}\n{}\n{}", start, src, end);
    }

    dest.to_string()
}