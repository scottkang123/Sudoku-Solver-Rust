
pub mod solver{

    use std::io::{BufRead, BufReader};
    use std::fs::File;

    const ROW_SIZE : usize = 9;
    const COL_SIZE : usize = 9;

    /**
    * Creates a 2-D array representation of the sudoku puzzle from the text file
    * Each element in the array is an integer between 0 to 9 that is copied from the input text-file puzzle
    *
    * @return   The 2-D array of size ROW_SIZE X COL-SIZE of element type usize
    */
    pub fn read_file() -> [[usize; COL_SIZE]; ROW_SIZE]{
        
        let mut grid : [[usize; COL_SIZE]; ROW_SIZE] = [[0; COL_SIZE]; ROW_SIZE];
    
        // modify the filepath here to test different sudoku puzzles. 
        let filepath = "txt/sudoku-test1.txt";
        let reader = BufReader::new(File::open(filepath).expect("open failed"));
    
        let upper_bound = 9;
        let lower_bound = 0;
        let mut r = 0;
        let mut c = 0;
    
        for line in reader.lines() {
            for ch in line.expect("characters are expected").chars() {
                if ch == ' '{
                    continue;
                }else{
                    let number : usize = ch.to_digit(10).unwrap() as usize;
                    if number >= lower_bound && number <= upper_bound{
                        grid[r][c] = number;
                        c = c + 1;
                    }
                }
            }
            c = 0;
            r = r + 1;
        }
        return grid;
    }

    /**
    * Checks if the num can fit in the corresponding location of the 2-D array board
    *
    * @param  board A 2-D array of size ROW_SIZE X COL_SIZE with elements of usize
    * @param  row  A row of type usize of the position to check.
    * @param  col  A col of type usize of the position to check
    * @param  num  A number of type usize to check if it can fit in the corresponding position
    * @return      A Boolean value of whether the num can fit in the corresponding position 
    */     
    pub fn valid(board: [[usize; COL_SIZE]; ROW_SIZE], row: usize, col: usize, num: usize) -> bool{

        //checks the column  
        for x in 0..9 {
            if board[row][x] == num{
                return false;
            }
        }

        //checks the row
        for y in 0..9 {
            if board[y][col] == num{
                return false;
            }
        }
        
        //checks the square 
        let x_index: usize = row / 3 * 3;
        let y_index: usize = col / 3 * 3;

        for x in 0..3 {
            for y in 0..3 {
                if board[x_index + x][y_index + y] == num {
                    return false;
                }
            }
        }

        return true;
    }

    /**
    * Prints the board into the console in a formatted way
    *
    * @param  board A 2-D array of size ROW_SIZE X COL_SIZE with elements of usize
    */
    pub fn print_board(board : [[usize; COL_SIZE]; ROW_SIZE] ){

        for x in 0..ROW_SIZE {

            if x % 3 == 0 && x != 0{
                println!("- - - - - - - - - - -");
            }

            for y in 0..COL_SIZE{
                if y % 3 == 0 && y != 0{
                    print!("| {} ", board[x][y]);
                }else{
                    print!("{} ", board[x][y]);
                }
            }
            println!();
            
        }

    }

    /**
    * Checks if the sudoku board is solved by checking if there is any zero remaining in the board
    *
    * @param  board A 2-D array of size ROW_SIZE X COL_SIZE with elements of usize
    * @return      A Boolean value of whether the board is solved or not
    */
    pub fn is_solved(board : [[usize; COL_SIZE]; ROW_SIZE]) -> bool{
        let next_cell : [usize; 2] = find_next_position(board);
        if next_cell == [9,9]{
            return true;
        }
        return false;
    }

    /**
    * Finds a position of next empty spot (0) in the board. The first element is the corresponding row
    * and the second element is the corresponding col
    *
    * @param  board A 2-D array of size ROW_SIZE X COL_SIZE with elements of usize
    * @return      An array of size 2 of type usize. 
    */
    pub fn find_next_position(board : [[usize; COL_SIZE]; ROW_SIZE]) -> [usize;2]{
        for r in 0..ROW_SIZE{
            for c in 0..COL_SIZE{
                if board[r][c] == 0{
                    return [r, c];
                }
            }
        }
        return [9,9];
    }

    /**
    * Returns the solved Sudoku Board. If not solvable, returns the original board
    *
    * @param  board A 2-D array of size ROW_SIZE X COL_SIZE with elements of usize
    * @return      The 2-D array of ROW_SIZE X COL_SIZE with element of type usize. 
    */
    pub fn solve(board : [[usize; COL_SIZE]; ROW_SIZE]) -> [[usize; COL_SIZE];ROW_SIZE]{

        let mut result = board;
        let empty_cell : [usize;2] = find_next_position(board);
        if empty_cell == [9, 9]{
            return result; 
        }

        let row = empty_cell[0];
        let col = empty_cell[1];

        for number in 1..10 {
            if valid(board, row, col, number){
                result[row][col] = number;
                result = solve(result);
                if find_next_position(result) == [9, 9]{
                    return result; 
                }
            }
        }
        result = board;
        return result;

    }

    

} 

/**
    * Main function for the program. Transform the given text-file formatted 
    * maze into the grid. Print the original board and then try to solve the puzzle
    * If the puzzle is solved, then prints the solved puzzle. If the puzzle is not 
    * solvable, then prints the message
*/
fn main(){
    let grid = solver::read_file();
    println!("\nPuzzle:\n");
    solver::print_board(grid);
    let result = solver::solve(grid);

    if grid == result{
        println!("\nThe Sudoku is Impossible to Solve!");
    }else{
        println!("\nSolution:\n");
        solver::print_board(result);
    }
    println!();
    
}