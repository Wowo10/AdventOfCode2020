pub enum Space {
    Tree,
    NotTree,
}

pub mod file_utils {
    use crate::functions::Space;

    pub fn get_input() -> Vec<Vec<Space>> {
        parse_input(load_file_by_lines())
    }

    pub fn pattern_length(stock: &Vec<Vec<Space>>) -> usize{
        stock[0].len()
    }

    fn load_file_by_lines() -> Vec<String> {
        use std::fs;
        use std::path::Path;

        let pathstr = String::from("data/input");
        let path = Path::new(&pathstr);

        let buffer = fs::read_to_string(path).expect("Failed to initialise file read.");

        buffer.lines().map(|line| line.to_owned()).collect()
    }

    fn parse_input(raw_input: Vec<String>) -> Vec<Vec<Space>> {
        raw_input
            .iter()
            .map(|x| {
                x.chars()
                    .map(|character| match character {
                        '#' => Space::Tree,
                        '.' => Space::NotTree,
                        _ => Space::NotTree,
                    })
                    .collect()
            })
            .collect()
    }
}
