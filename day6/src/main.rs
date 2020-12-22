mod functions;

fn main() {
    let input = functions::file_utils::get_input();

    let mut counter = 0;
    for elem in &input {
        counter += count_unique_characters(|c| elem.any_answered(c));
    }

    println!("Result1: {}", counter);
    counter = 0;
    for elem in &input {
        counter += count_unique_characters(|c| elem.all_answered(c));
    }

    println!("Result2: {}", counter);
}

fn count_unique_characters<F>(f: F) -> i32
where
    F: Fn(char) -> bool,
{
    let mut counter = 0;
    for i in 0..26 {
        let c = add_char('a', i);

        if f(c) {
            counter += 1;
        }
    }
    counter
}

fn add_char(c: char, u: u32) -> char {
    std::char::from_u32(c as u32 + u).unwrap_or(c)
}
