mod functions;

fn main() {
    let input = functions::file_utils::get_input();

    let mut counter = 0;
    for elem in &input {
        counter += count_unique_characters(elem);
    }

    println!("Result: {}", counter)
}

fn count_unique_characters(input: &str) -> i32 {
    let mut counter = 0;
    for i in 0..26 {
        let c = add_char('a', i);

        if input.contains(c) {
            counter += 1;
        }
    }
    
    counter
}

fn add_char(c: char, u: u32) -> char {
    std::char::from_u32(c as u32 + u).unwrap_or(c)
}
