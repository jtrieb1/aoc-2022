use crate::util::{parse_lines_into, read_input_to_str, AOCSolution};
use std::collections::HashMap;
use std::str::FromStr;

solution!(Day 7 => FileSystem);

#[derive(Debug)]
pub struct FileSystem {
    root: Directory,
    path: Vec<String>,
    journal: Journal,
}

impl AOCSolution for FileSystem {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }

    fn part_1(&mut self) -> String {
        self.parse_journal();
        let sizes = self.get_dir_sizes_under_threshold(100000);
        let total: u32 = sizes.iter().sum();
        format!("{}", total)
    }

    fn part_2(&mut self) -> String {
        let unused_space = self.get_unused_space();
        let desired_space: u32 = 30000000;
        let threshold = desired_space - unused_space;
        let sizes = self.get_dir_sizes_over_threshold(threshold);
        let value = sizes.iter().min().unwrap();
        format!("{}", value)
    }
}

impl FileSystem {
    pub fn new_from_file(input_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_file_path, true)?;
        Self::new_from_str(&input_str)
    }

    pub fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let entries = parse_lines_into::<TerminalLine>(input_str)?;
        let root = Directory::new();

        Ok(Self {
            root,
            path: Vec::new(),
            journal: Journal { lines: entries },
        })
    }

    fn parse_journal(&mut self) {
        if self.root.files.len() != 0 && self.root.subdirs.len() != 0 {
            return;
        }
        let lines = self.journal.lines.clone();
        for line in lines.iter().skip(1) {
            self.parse_line(line);
        }
        self.path = vec![];
    }

    fn parse_line(&mut self, line: &TerminalLine) {
        if self.path.len() == 0 {
            self.handle_empty_path(line);
            return;
        }
        self.process_from_cwd(line);
    }

    fn handle_empty_path(&mut self, line: &TerminalLine) {
        let mut cwd = self.root.clone();
        self.parse_line_from(&mut cwd, line);
        self.root = cwd;
    }

    fn get_cwd(&mut self) -> Option<&mut Box<Directory>> {
        let path = self.path.clone();
        self.get_dir_at_path(&path)
    }
    
    fn process_from_cwd(&mut self, line: &TerminalLine) {
        if let None = self.get_cwd() { return; }
        let old_path = self.path.clone();
        let mut cwd = self.get_cwd().unwrap().clone();
        self.parse_line_from(&mut cwd, line);
        let cwd_ref = self.get_dir_at_path(&old_path).unwrap();
        *cwd_ref = cwd;
    }
    
    fn get_dir_at_path(&mut self, path: &Vec<String>) -> Option<&mut Box<Directory>> {
        let mut cwd = self.root.subdirs.get_mut(path.first().unwrap());
        for dirname in path.iter().skip(1) {
            let unwrapped_cwd = cwd.unwrap();
            cwd = unwrapped_cwd.subdirs.get_mut(dirname);
        }
        cwd
    }

    fn parse_line_from(&mut self, cwd: &mut Directory, line: &TerminalLine) {
        match line {
            TerminalLine::Command(cmd) => {
                self.parse_cmd(cmd);
            }
            TerminalLine::Output(out) => match out {
                TerminalOutput::Dir(dirname) => cwd.append_dir(&dirname),
                TerminalOutput::File(f) => cwd.append_file(f.clone()),
            },
        }
    }

    fn parse_cmd(&mut self, cmd: &TerminalCommand) {
        match cmd {
            TerminalCommand::LS => {}
            TerminalCommand::CD(dirname) => {
                if dirname == ".." {
                    self.path.pop();
                    return;
                }
                self.path.push(dirname.clone());
            }
        }
    }

    fn get_dir_sizes(&self) -> Vec<u32> {
        self.root.get_subdir_sizes()
    }

    fn get_dir_sizes_under_threshold(&self, threshold: u32) -> Vec<u32> {
        self.get_dir_sizes()
            .into_iter()
            .filter(|s| *s <= threshold)
            .collect()
    }

    fn get_unused_space(&self) -> u32 {
        let total_space: u32 = 70000000;
        let dir_sizes = self.get_dir_sizes();
        let used_space = dir_sizes.iter().max().unwrap();
        total_space - used_space
    }

    fn get_dir_sizes_over_threshold(&self, threshold: u32) -> Vec<u32> {
        self.get_dir_sizes()
            .into_iter()
            .filter(|s| *s >= threshold)
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Directory {
    files: Vec<File>,
    subdirs: HashMap<String, Box<Directory>>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            subdirs: HashMap::new(),
        }
    }

    pub fn append_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn append_dir(&mut self, dir: &str) {
        self.subdirs
            .insert(dir.to_string(), Box::new(Directory::new()));
    }

    pub fn total_size(&self) -> u32 {
        self.subdirs.values().map(|v| v.total_size()).sum::<u32>()
            + self.files.iter().map(|f| f.size).sum::<u32>()
    }

    pub fn get_subdir_sizes(&self) -> Vec<u32> {
        let this_size = vec![self.total_size()];
        let mut subsizes: Vec<u32> = self
            .subdirs
            .values()
            .map(|d| d.get_subdir_sizes())
            .flatten()
            .collect();
        subsizes.extend(this_size);
        subsizes
    }
}

#[derive(Clone, Debug)]
struct File {
    size: u32,
}

#[derive(Debug)]
struct Journal {
    lines: Vec<TerminalLine>,
}

#[derive(Clone, Debug)]
enum TerminalLine {
    Command(TerminalCommand),
    Output(TerminalOutput),
}

impl FromStr for TerminalLine {
    type Err = TerminalLineParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() && s.chars().nth(0).unwrap() == '$' {
            Ok(TerminalLine::Command(s.parse::<TerminalCommand>()?))
        } else {
            Ok(TerminalLine::Output(s.parse::<TerminalOutput>()?))
        }
    }
}

custom_error!(TerminalLineParseErr);

impl From<TerminalCommandParseErr> for TerminalLineParseErr {
    fn from(e: TerminalCommandParseErr) -> Self {
        TerminalLineParseErr(e.0)
    }
}

impl From<TerminalOutputParseErr> for TerminalLineParseErr {
    fn from(e: TerminalOutputParseErr) -> Self {
        TerminalLineParseErr(e.0)
    }
}

#[derive(Clone, Debug)]
enum TerminalCommand {
    CD(String),
    LS,
}

impl FromStr for TerminalCommand {
    type Err = TerminalCommandParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections = s.split(' ').collect::<Vec<&str>>();
        if sections.is_empty() || sections[0] != "$" {
            return Err(TerminalCommandParseErr(
                "Invalid command: either empty or without cursor.",
            ));
        }
        match sections[1] {
            "cd" => Ok(TerminalCommand::CD(sections[2].to_string())),
            "ls" => Ok(TerminalCommand::LS),
            _ => Err(TerminalCommandParseErr("Unknown command")),
        }
    }
}

custom_error!(TerminalCommandParseErr);

#[derive(Clone, Debug)]
enum TerminalOutput {
    Dir(String),
    File(File),
}

impl FromStr for TerminalOutput {
    type Err = TerminalOutputParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections = s.split(' ').collect::<Vec<&str>>();
        if sections.len() != 2 {
            return Err(TerminalOutputParseErr(
                "Invalid output line: expected len 2",
            ));
        }
        if sections[0] == "dir" {
            return Ok(TerminalOutput::Dir(sections[1].to_string()));
        }
        if let Ok(size) = sections[0].parse::<u32>() {
            return Ok(TerminalOutput::File(File { size }));
        }
        Err(TerminalOutputParseErr("Invalid output line."))
    }
}

custom_error!(TerminalOutputParseErr);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_test() {
        let input_str = "
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
        ";

        let mut fs = FileSystem::new_from_str(input_str).expect("Uh oh");
        fs.parse_journal();
        assert_eq!(fs.part_1(), "95437");
        assert_eq!(fs.part_2(), "24933642");
    }
}
