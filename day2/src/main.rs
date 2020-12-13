mod functions;
use functions::file_utils::*;

fn main() {
    let input = get_input();
    let mut counter = 0;
    
    for password in &input {
        if password.is_valid() {
            counter +=1;
        }
    }
    println!("The result is: {}", counter);
    counter = 0;
    
    for password in &input {
        if password.is_valid2() {
            counter +=1;
        }
    }

    println!("The result is: {}", counter);
}
