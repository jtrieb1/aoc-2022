pub fn read_input_to_str(input_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let input_contents = std::fs::read_to_string(input_path)?;
    Ok(input_contents)
}