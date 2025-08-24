use std::io;

fn main() {
    let gridSize = getGridSize();
    generateSudokuGrid(gridSize);
}

fn getGridSize() -> i32 {
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

fn generateSudokuGrid(gridSize: i32) {

}
