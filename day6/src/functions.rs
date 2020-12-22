extern crate regex;
pub struct Group {
    answers: Vec<String>,
}

impl Group {
    pub fn create(chunk: &str) -> Self {
        let mut v = Vec::new();
        for elem in chunk.lines() {
            v.push(String::from(elem));
        }

        Group { answers: v }
    }

    pub fn any_answered(&self, c: char) -> bool {
        for elem in &self.answers {
            if elem.contains(c) {
                return true;
            }
        }

        false
    }
    
    pub fn all_answered(&self, c: char) -> bool {
        for elem in &self.answers {
            if !elem.contains(c) {
                return false;
            }
        }

        true
    }
}

pub mod file_utils {
    use crate::functions::Group;
    pub fn get_input() -> Vec<Group> {
        parse_input(load_file_by_passport_chunks())
    }

    fn load_file_by_passport_chunks() -> Vec<String> {
        use std::fs;
        use std::path::Path;

        let pathstr = String::from("data/input");
        let path = Path::new(&pathstr);

        let buffer = fs::read_to_string(path).expect("Failed to initialise file read.");

        let empty_line_regex = regex::Regex::new(r"\s\n").unwrap();

        empty_line_regex
            .split(&buffer)
            .map(|str| String::from(str))
            .collect()
    }

    fn parse_input(raw_input: Vec<String>) -> Vec<Group> {
        raw_input
            .iter()
            .map(|x| Group::create(x))
            .collect()
    }
}
