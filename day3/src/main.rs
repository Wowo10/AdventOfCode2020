mod functions;
use functions::file_utils::*;
use functions::Space;

fn main() {
    let input = get_input();

    // println!("Result (1,1): {}", slope(&input, 1,1));
    // println!("Result (3,1): {}", slope(&input, 3,1));
    // println!("Result (5,1): {}", slope(&input, 5,1));
    // println!("Result (7,1): {}", slope(&input, 7,1));
    // println!("Result (1,2): {}", slope(&input, 1,2));

    println!("Resul: {}", slope(&input, 1,1) * slope(&input, 3,1) * slope(&input, 5,1) * slope(&input, 7,1) * slope(&input, 1,2));
}

fn slope(stock: &Vec<Vec<Space>>, right: usize, down: usize) -> usize{
    
    let stock_length = stock.len();
    let stock_width = pattern_length(&stock);

    let mut slope_counter = 0;
    let mut tree_counter = 0;
    for i in 0..stock_length/down {
        
        match stock[i*down][slope_counter] {
            Space::NotTree => {},
            Space::Tree => {
                tree_counter += 1;                
            }
        };

        slope_counter += right;
        slope_counter %= stock_width;
    }

    tree_counter
}
