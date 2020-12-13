extern crate regex;
use regex::Regex;

pub struct Passport {
    pub birth_year: String,
    pub issue_year: String,
    pub expiration_year: String,
    pub height: String,
    pub hair_color: String,
    pub eye_color: String,
    pub passport_id: String,
    pub country_id: String,
}

impl Passport {
    fn empty() -> Self {
        Passport {
            birth_year: String::new(),
            issue_year: String::new(),
            expiration_year: String::new(),
            height: String::new(),
            hair_color: String::new(),
            eye_color: String::new(),
            passport_id: String::new(),
            country_id: String::new(),
        }
    }

    pub fn parse(input: &str) -> Self {
        let regex = Regex::new(r"(\w{1,}):(#{0,1}\w{1,})").unwrap();
        let mut pass = Passport::empty();

        for cap in regex.captures_iter(&input) {
            match &cap[1] {
                "byr" => {
                    pass.birth_year = String::from(&cap[2]);
                }
                "iyr" => {
                    pass.issue_year = String::from(&cap[2]);
                }
                "eyr" => {
                    pass.expiration_year = String::from(&cap[2]);
                }
                "hgt" => {
                    pass.height = String::from(&cap[2]);
                }
                "hcl" => {
                    pass.hair_color = String::from(&cap[2]);
                }
                "ecl" => {
                    pass.eye_color = String::from(&cap[2]);
                }
                "pid" => {
                    pass.passport_id = String::from(&cap[2]);
                }
                "cid" => {
                    pass.country_id = String::from(&cap[2]);
                }
                _ => {
                    panic!("Broken Record: {}, {}", &cap[1], &cap[2]);
                }
            }
        }

        pass
    }

    pub fn is_valid(&self) -> bool {
        !(self.birth_year.is_empty()
            || self.expiration_year.is_empty()
            || self.eye_color.is_empty()
            || self.hair_color.is_empty()
            || self.height.is_empty()
            || self.issue_year.is_empty()
            || self.passport_id.is_empty())
    }
    pub fn is_valid2(&self) -> bool {
        self.is_valid()
            && self.valid_birth_year()
            && self.valid_issue_year()
            && self.valid_expiration_year()
            && self.valid_height()
            && self.valid_hair_color()
            && self.valid_eye_color()
            && self.valid_passport_id()
    }

    fn valid_birth_year(&self) -> bool {
        if let Ok(birth_year) = self.birth_year.parse::<i32>() {
            birth_year >= 1920 && birth_year <= 2002
        } else {
            false
        }
    }

    fn valid_issue_year(&self) -> bool {
        if let Ok(issue_year) = self.issue_year.parse::<i32>() {
            issue_year >= 2010 && issue_year <= 2020
        } else {
            false
        }
    }

    fn valid_expiration_year(&self) -> bool {
        if let Ok(expiration_year) = self.expiration_year.parse::<i32>() {
            expiration_year >= 2020 && expiration_year <= 2030
        } else {
            false
        }
    }

    fn valid_height(&self) -> bool {
        let regex = Regex::new(r"(\d{2,3})(cm|in)").unwrap();

        if let Some(cap) = regex.captures_iter(&self.height).next() {
            match &cap[2] {
                "cm" => {
                    let height: i32 = cap[1].parse().unwrap();
                    height >= 150 && height <= 193
                }
                "in" => {
                    let height: i32 = cap[1].parse().unwrap();
                    height >= 59 && height <= 76
                }
                _ => false,
            }
        } else {
            false
        }
    }
    fn valid_hair_color(&self) -> bool {
        let regex = Regex::new(r"#[0-9a-fA-F]{6}").unwrap();

        regex.is_match(&self.hair_color) && self.hair_color.len() == 7
    }

    fn valid_eye_color(&self) -> bool {
        let regex = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();

        regex.is_match(&self.eye_color)
    }

    fn valid_passport_id(&self) -> bool {
        let regex = Regex::new(r"\d{9}").unwrap();

        regex.is_match(&self.passport_id) && self.passport_id.len() == 9
    }
}

pub mod file_utils {
    use crate::functions::Passport;

    pub fn get_input() -> Vec<Passport> {
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

    fn parse_input(raw_input: Vec<String>) -> Vec<Passport> {
        raw_input.iter().map(|x| Passport::parse(&x)).collect()
    }
}
