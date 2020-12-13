extern crate regex;
use regex::Regex;

pub struct PasswordWithRules {
    pub password: String,
    pub rule_character: char,
    pub rule_min_amount: usize,
    pub rule_max_amount: usize,
}

impl PasswordWithRules {
    pub fn parse(input: &str) -> Self {
        let regex = Regex::new(r"(\d{1,2})-(\d{1,2})\s(\D):\s(.*)").unwrap();

        let cap = regex.captures_iter(&input).next().unwrap();

        PasswordWithRules {
            password: String::from(&cap[4]),
            rule_character: cap[3].chars().next().unwrap(),
            rule_min_amount: cap[1].parse().unwrap(),
            rule_max_amount: cap[2].parse().unwrap(),
        }
    }

    pub fn is_valid(&self) -> bool {
        let count = self.password.matches(self.rule_character).count();

        count >= self.rule_min_amount && count <= self.rule_max_amount
    }

    pub fn is_valid2(&self) -> bool {
        let mut valid = false;

        if self.password.chars().nth(self.rule_min_amount - 1) == Some(self.rule_character) {
            valid = !valid;
        } 

        if self.password.chars().nth(self.rule_max_amount - 1) == Some(self.rule_character) {
            valid = !valid;
        } 

        valid
    }
}

pub mod file_utils {
    use crate::functions::PasswordWithRules;

    pub fn get_input() -> Vec<PasswordWithRules> {
        parse_input(load_file_by_lines())
    }

    fn load_file_by_lines() -> Vec<String> {
        use std::fs;
        use std::path::Path;

        let pathstr = String::from("data/input");
        let path = Path::new(&pathstr);

        let buffer = fs::read_to_string(path).expect("Failed to initialise file read.");

        buffer.lines().map(|line| line.to_owned()).collect()
    }

    fn parse_input(raw_input: Vec<String>) -> Vec<PasswordWithRules> {
        raw_input
            .iter()
            .map(|x| PasswordWithRules::parse(x))
            .collect()
    }
}
