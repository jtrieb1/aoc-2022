use std::str::FromStr;

pub fn read_input_to_str(input_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let input_contents = std::fs::read_to_string(input_path)?;
    let input_contents = input_contents.trim().to_string();
    Ok(input_contents)
}

pub fn convert_str_to_sections(input: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let input_str = input.trim();
    let mut sections: Vec<String> = Vec::new();
    for chunk in input_str.split("\n\n") {
        if chunk.is_empty() {
            continue;
        }
        sections.push(chunk.trim().to_string());
    }
    Ok(sections)
}

pub fn parse_lines_into<T>(lines: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: FromStr,
    <T as FromStr>::Err: 'static + std::error::Error,
{
    let mut nums: Vec<T> = Vec::new();
    for line in lines.split('\n') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let num = line.parse::<T>()?;
        nums.push(num);
    }
    Ok(nums)
}
