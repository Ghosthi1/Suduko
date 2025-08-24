use std::io;

fn main() {
    let grid_size = get_grid_size();

    // creates a dynamic vector 
    let grid: Vec<Vec<i32>> = vec![vec![0; grid_size as usize]; grid_size as usize];
    
    // initializes the Sudoku grid displaying blank values
    generate_sudoku_grid_init(&grid);
}

// Gets grid from user
fn get_grid_size() -> i32 {
    println!("Enter the size of the Sudoku grid (e.g., 9 for a 9x9 grid):");

    //sets a mutable string 
    let mut input = String::new();

    io::stdin()
        //reads line and appends to string ,&mut creates mutable reference
        .read_line(&mut input)
        // error handling
        .expect("Failed to read line");

    let number: i32 = input
        .trim() //removes whitespace
        .parse() //parses string to number
        .expect("Please type a number!"); //error handling

    return number;

}

//displays the grid to user 
fn generate_sudoku_grid_init(grid_size: &Vec<Vec<i32>>) {

}

