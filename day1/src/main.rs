mod functions;
use functions::file_utils::*;

fn main() {
    let input = get_input();

    match functions::counting::find_two(&input) {
        Some((first, second)) => println!(
            "Values are {}, {}, product: {}",
            first,
            second,
            first * second
        ),

        None => println!("No such 2 values"),
    }

    match functions::counting::find_three(&input) {
        Some((first, second, third)) => println!(
            "Values are {}, {}, {}, product: {}",
            first,
            second,
            third,
            first * second * third
        ),

        None => println!("No such 2 values"),
    }
}
