mod functions;

fn main() {
    let mut input = functions::file_utils::get_input();

    let mut max_id = 0;
    for pat in &input {
        let id = pat.id();
        if id > max_id {
            max_id = id;
        }
    }

    println!("Result: {}", max_id);

    input.sort_by(|x, y| x.id().cmp(&y.id()));
    
    let mut prev = input.first().unwrap().id();
    for pat in input.iter().skip(1) {
        if pat.id() -1 == prev
        {
            prev = pat.id();
        }
        else{
            break;
        }
    }

    println!("My seat: {}", prev+1);
}
