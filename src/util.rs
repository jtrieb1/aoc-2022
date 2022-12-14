use std::str::FromStr;

pub type SResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait AOCSolution {
    fn load_from(input_file_path: &str) -> SResult<Box<Self>>
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

macro_rules! custom_error {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name(&'static str);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{}", self.0))
            }
        }

        impl std::error::Error for $name {}
    }
}

pub fn read_input_to_str(input_path: &str, trim: bool) -> SResult<String> {
    let mut input_contents = std::fs::read_to_string(input_path)?;
    if trim {
        input_contents = input_contents.trim().to_string();
    }
    Ok(input_contents)
}

pub fn convert_str_to_sections(mut input: &str, trim: bool) -> SResult<Vec<String>> {
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

pub fn parse_lines_into<T>(lines: &str) -> SResult<Vec<T>>
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

// Width, Height
pub fn str_to_grid_info(input: &str) -> (usize, usize) {
    let input = input.trim();
    let mut lines = input.lines();
    let width = lines.nth(0).map(|l| l.len()).unwrap();
    lines = input.lines();
    let height = lines.count();
    (width, height)
}

pub fn parse_each_char<T>(s: &str) -> SResult<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: 'static + std::error::Error
{
    let mut res: Vec<T> = Vec::new();
    for c in s.split("") {
        if c.is_empty() || c == "\n" { continue; }
        res.push(c.parse::<T>()?);
    }
    Ok(res)
}