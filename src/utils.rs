pub fn parse_items(input: &str) -> Vec<String> {
    input.trim().split(',')
        .map(|s| s.trim().to_string())
        .collect()
}