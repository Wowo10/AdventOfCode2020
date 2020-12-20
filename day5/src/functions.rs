pub struct Seat {
    pub code: String,
}

impl Seat {
    pub fn create(code: &str) -> Self {
        Seat {
            code: String::from(code),
        }
    }

    pub fn row(&self) -> i32 {
        let mut min_counter = 0;
        let mut max_counter = 128;

        for sign in self.code.chars().take(7) {
            let difference = max_counter - min_counter;
            match sign {
                'F' => {
                    max_counter -= difference / 2;
                }
                'B' => {
                    min_counter += difference / 2;
                }
                _ => panic!("Unknown character while reading row!"),
            }
        }
        min_counter
    }

    pub fn column(&self) -> i32 {
        let mut min_counter = 0;
        let mut max_counter = 8;

        for sign in self.code.chars().skip(7).take(3) {
            let difference = max_counter - min_counter;
            match sign {
                'L' => {
                    max_counter -= difference / 2;
                }
                'R' => {
                    min_counter += difference / 2;
                }
                _ => panic!("Unknown character while reading row!"),
            }
        }

        min_counter
    }

    pub fn id(&self) -> i32 {
        self.row() * 8 + self.column()
    }
}

pub mod file_utils {
    use crate::functions::Seat;

    pub fn get_input() -> Vec<Seat> {
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

    fn parse_input(raw_input: Vec<String>) -> Vec<Seat> {
        raw_input.iter().map(|x| Seat::create(x)).collect()
    }
}
