extern crate regex;

pub mod file_utils {
    use regex::Regex;
    pub fn get_input() -> Vec<String> {
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

    fn parse_input(raw_input: Vec<String>) -> Vec<String> {
        let whitespace_regex: Regex = Regex::new("\\s").unwrap();

        raw_input.iter().map(|x| String::from(whitespace_regex.replace_all(x, ""))).collect()
    }
}
