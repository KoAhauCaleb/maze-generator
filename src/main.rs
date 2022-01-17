use std::io;
use rand::Rng;

//Grid size constants
const GRID_MULT: u32 = 3;
const GRID_X_MULT: u32 = 3;
const GRID_Y_MULT: u32 = 5;

const REF_COORD: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

fn main() {

    //Get the maze difficulty the user wants generated.
    println!("Enter maze difficulty (1-5): ");
    
    let mut difficulty = String::new();

    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line.");

    let difficulty: u32 = difficulty.trim().parse().expect("Please type a number.");

    generate_maze(difficulty);
    
}

fn generate_maze(difficulty: u32){
    let mut grid = create_empty_grid(difficulty);
    generate_with_recusive_randomized_depth_first(&mut grid);

    for row in 0..grid.len() {
        for cell_row in 0..3{
            for column in 0..grid[row].len() {
                if cell_row == 0{
                    print!("x");
                    if grid[row][column][0]{
                        print!("x");
                    } else {
                        print!(" ");
                    }
                    print!("x");
                } else if cell_row == 1{
                    if grid[row][column][3]{
                        print!("x");
                    } else {
                        print!(" ");
                    }
                    print!(" ");
                    if grid[row][column][1]{
                        print!("x");
                    } else {
                        print!(" ");
                    }
                    
                } else {
                    print!("x");
                    if grid[row][column][2]{
                        print!("x");
                    } else {
                        print!(" ");
                    }
                    print!("x");
                }
            }
            println!("")
        }
        
    }
}

fn generate_with_recusive_randomized_depth_first(grid: &mut Vec<Vec<[bool; 5]>>) {
    //Recursivley generate maze.
    rdf(grid, 0, 0);
}


fn rdf(grid: &mut Vec<Vec<[bool; 5]>>, cell_x: u32, cell_y: u32) {
    let mut adj_unvisited = count_adj_unvisited(grid, cell_x, cell_y);
    
    {
        let mut cell_x: usize = cell_x.try_into().unwrap();
        let mut cell_y: usize = cell_y.try_into().unwrap();
        grid[cell_x][cell_y][4] = true;
    }
    if adj_unvisited[4] {
        println!("This is a dead end ({}{})", cell_x, cell_y);
    }
    while adj_unvisited[4] {
        println!("Moved to cell {}/{}-------------------------------------------------------------------------------", cell_x, cell_y);
        println!("visited cells Top: {}, Right: {}, Bottom: {}, Left: {}", adj_unvisited[0], adj_unvisited[1], adj_unvisited[2], adj_unvisited[3]);
        for row in 0..grid.len() {
            for column in 0..grid[row].len() {
                if grid[row][column][4]{
                    print!("x");
                }
                else{
                    print!("o");
                }
            }
            println!("")
        }

        let mut selected_cell: usize = rand::thread_rng().gen_range(0..4);
        
        while adj_unvisited[selected_cell] {
            println!("Checked cell: {}", selected_cell);
            selected_cell = rand::thread_rng().gen_range(0..4);
        }

        println!("Selected cell: {}", selected_cell);

        //Get coords of next cell.
        let mut cell_x: i32 = cell_x.try_into().unwrap();
        let mut cell_y: i32 = cell_y.try_into().unwrap();
        let next_cell_x: u32 = (cell_x + REF_COORD[selected_cell][0]).try_into().unwrap();
        let next_cell_y: u32 = (cell_y + REF_COORD[selected_cell][1]).try_into().unwrap();

        //Open walls based on selection.
        {
            let mut cell_x: usize = cell_x.try_into().unwrap();
            let mut cell_y: usize = cell_y.try_into().unwrap();
            let mut next_cell_x: usize = next_cell_x.try_into().unwrap();
            let mut next_cell_y: usize = next_cell_y.try_into().unwrap();
            //Open wall in this cell.
            println!("-------Opened wall {} on this cell({}:{}).----------", selected_cell, cell_x, cell_y);
            grid[cell_x][cell_y][selected_cell] = false;
            //Open wall in next cell.
            let next_selected_wall = (selected_cell + 2) % 4;
            println!("-------Opened wall {} on next cell({}:{}).----------", next_selected_wall, next_cell_x, next_cell_y);
            grid[next_cell_x][next_cell_y][next_selected_wall] = false;
        }
        //Call rdf on next cell
        rdf(grid, next_cell_x, next_cell_y);

        //Check if cell still has placed next to it that are still unvisited.
        let mut cell_x: u32 = cell_x.try_into().unwrap();
        let mut cell_y: u32 = cell_y.try_into().unwrap();
        adj_unvisited = count_adj_unvisited(grid, cell_x, cell_y);
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++Backed up");
    }
}

fn count_adj_unvisited(grid: &mut Vec<Vec<[bool; 5]>>, cell_x: u32, cell_y: u32) -> [bool; 5] {
    let mut cell_x: usize = cell_x.try_into().unwrap();
    let mut cell_y: usize = cell_y.try_into().unwrap();
    
    
    let mut adj_unvisited = false;

    let (mut cell_up, mut cell_right, mut cell_down, mut cell_left) = (true, true, true, true);

    //cell up
    if cell_x > 0 {
        print!("{}, ", grid[cell_x - 1][cell_y][4]);
        if grid[cell_x - 1][cell_y][4] == false{
            adj_unvisited = true;
            cell_up = false;
        }
    }
    //cell right
    if (cell_y + 1) < grid[1].len().try_into().unwrap() {
        print!("{}, ",grid[cell_x][cell_y + 1][4]);
        if grid[cell_x][cell_y + 1][4] == false{
            adj_unvisited = true;
            cell_right = false;
        }
    }
    //cell down
    if (cell_x + 1) < grid.len().try_into().unwrap() {
        print!("{}, ", grid[cell_x + 1][cell_y][4]);
        if grid[cell_x + 1][cell_y][4] == false{
            adj_unvisited = true;
            cell_down = false;
        }
    }
    //cell left
    if cell_y > 0 {
        println!("{} ", grid[cell_x][cell_y - 1][4]);
        if grid[cell_x][cell_y - 1][4] == false{
            adj_unvisited = true;
            cell_left = false;
        }
    }

    [cell_up, cell_right, cell_down, cell_left, adj_unvisited]
}

fn create_empty_grid(difficulty: u32) -> Vec<Vec<[bool; 5]>> {

    //Calculate grid size based on difficulty
    let grid_size_x = difficulty * GRID_MULT * GRID_X_MULT;
    let grid_size_y = difficulty * GRID_MULT * GRID_Y_MULT;

    //Create 2 dimensional vector holding arrays to represent grid, arrays represent where opening in each
    //cell is at and if the the cell has been visited.
    let mut grid: Vec<Vec<[bool; 5]>> = Vec::new();

    //Create the grid.
    for _row in 0..grid_size_x {
        let mut row_vec: Vec<[bool; 5]> = Vec::new();
        for _column in 0..grid_size_y {
            row_vec.push([true, true, true, true, false]); //0: top wall; 1: right wall; 2: bottom wall; 3: left wall; 4: visited
        }
        grid.push(row_vec)
    }

    //Show the grid and the first wall in each cell, this is for testin
    for row in &grid {
        for column in row {
            print!("{}", column[0]);
        }
        println!("")
    }

    println!("--------------------------------------------");
    
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            print!("{}", grid[row][column][0]);
        }
        println!("")
    }

    grid
}
