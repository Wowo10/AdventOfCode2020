mod functions;

fn main() {
    let input = functions::file_utils::get_input();

    println!("Done getting Input");

    println!("Result: {}", input.iter().filter(|&n| n.is_valid()).count());
    println!(
        "Result2: {}",
        input.iter().filter(|&n| n.is_valid2()).count()
    );
}