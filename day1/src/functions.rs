pub mod file_utils {
    pub fn get_input() -> Vec<i32> {
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

    fn parse_input(raw_input: Vec<String>) -> Vec<i32> {
        raw_input.iter().map(|x| x.parse().unwrap()).collect()
    }
}

pub mod counting {
    pub fn find_two(input: &Vec<i32>) -> Option<(i32, i32)> {
        for (outer_index, outer_value) in input.iter().enumerate() {
            for (inner_index, inner_value) in input.iter().enumerate() {
                if outer_index == inner_index {
                    continue;
                }
                if outer_value + inner_value == 2020 {
                    return Some((outer_value.clone(), inner_value.clone()));
                }
            }
        }

        None
    }

    pub fn find_three(input: &Vec<i32>) -> Option<(i32, i32, i32)> {
        for (outer_index, outer_value) in input.iter().enumerate() {
            for (middle_index, middle_value) in input.iter().enumerate() {
                for (inner_index, inner_value) in input.iter().enumerate() {
                    if outer_index == inner_index || outer_index == middle_index || inner_index == middle_index{
                        continue;
                    }
                    if outer_value + inner_value + middle_value == 2020 {
                        return Some((outer_value.clone(), middle_value.clone(), inner_value.clone()));
                    }
                }
            }
        }

        None
    }
}
