use std::str::FromStr;

pub trait AOCSolution {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized;
        
    fn part_1(&mut self) -> String {
        "TODO!".to_string()
    }
    fn part_2(&mut self) -> String {
        "TODO!".to_string()
    }
}

macro_rules! solution {
    (Day $day:expr => $s:ty) => {
        pub fn solution() -> Result<Box<$s>, Box<dyn std::error::Error>> {
            <$s>::load_from(&format!("input/day{}.txt", $day))
        }
    };
}

pub fn read_input_to_str(input_path: &str, trim: bool) -> Result<String, Box<dyn std::error::Error>> {
    let mut input_contents = std::fs::read_to_string(input_path)?;
    if trim {
        input_contents = input_contents.trim().to_string();
    }
    Ok(input_contents)
}

pub fn convert_str_to_sections(mut input: &str, trim: bool) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if trim {
        input = input.trim();
    }
    let mut sections: Vec<String> = Vec::new();
    for mut chunk in input.split("\n\n") {
        if chunk.is_empty() {
            continue;
        }
        if trim {
            chunk = chunk.trim();
        }
        sections.push(chunk.to_string());
    }
    Ok(sections)
}

pub fn parse_lines_into<T>(lines: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: FromStr,
    <T as FromStr>::Err: 'static + std::error::Error,
{
    let mut all_parsed: Vec<T> = Vec::new();
    for line in lines.split('\n') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parsed = line.parse::<T>()?;
        all_parsed.push(parsed);
    }
    Ok(all_parsed)
}
