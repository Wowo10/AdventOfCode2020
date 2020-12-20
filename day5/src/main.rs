mod functions;

fn main() {
    let input = functions::file_utils::get_input();

    let mut max_id = 0;
    for pat in input {
        let id = pat.id();
        if id > max_id {
            max_id = id;
        }
    }

    println!("Result: {}", max_id);
}
