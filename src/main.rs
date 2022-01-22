use std::io;
use rand::Rng;

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

//Grid size constants
const GRID_MULT: u32 = 3;
const GRID_X_MULT: u32 = 3;
const GRID_Y_MULT: u32 = 5;

//Maps relative position of selected cell
const REF_COORD: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

//SVG constants
const TOP_MARGIN: usize = 100;
const BOTTOM_MARGIN: usize = 100;
const LEFT_MARGIN: usize = 100;
const RIGHT_MARGIN: usize = 100;

fn main() {

    //Get the maze difficulty the user wants generated.
    //Debug can only support up to 5 difficulty.
    println!("Enter maze difficulty (1-10): ");
    
    let mut difficulty = String::new();

    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line.");

    let difficulty: u32 = difficulty
        .trim().parse()
        .expect("Please type a number.");

    //Generate maze with selected difficulty.
    generate_maze(difficulty);
}

fn generate_maze(difficulty: u32){
    
    //Create an 2d vector filled with maze cells with no connections.
    let mut grid = create_empty_grid(difficulty);
    
    generate_with_recursive_randomized_depth_first(&mut grid);

    create_svg(grid, difficulty);
}

fn create_svg(grid: Vec<Vec<[bool; 5]>>, difficulty: u32) {
    
    //Calculate settings for the svg file.
    let cell_size = 100;
    let svg_width = cell_size * difficulty * GRID_MULT * GRID_Y_MULT + 200;
    let svg_height = cell_size * difficulty * GRID_MULT * GRID_X_MULT + 200;
    let svg_height: i32 = svg_height.try_into().unwrap();

    //Create a path that with surround the maze.
    let mut data = Data::new()
        .move_to((0, 0))
        .line_by((0, svg_height))
        .line_by((svg_width, 0))
        .line_by((0, svg_height * -1))
        .close();

    //Go through each cell and create a line where it's walls are at.
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {

            //This calculates the top left coordinates of the cell
            //according to desired margin width and cell size.
            let top_left_x = row * 100 + TOP_MARGIN;
            let top_left_y = column * 100 + LEFT_MARGIN;
            
            //The next 4 if statements checks each wall in a cell and draws one
            //if its there.
            if grid[row][column][0]{
                data = data.move_to((top_left_y, top_left_x))
                    .line_by((100, 0))
                    .close();
            }
            if grid[row][column][1]{
                data = data.move_to((top_left_y + 100, top_left_x))
                    .line_by((0, 100))
                    .close();
            }
            if grid[row][column][2]{
                data = data.move_to((top_left_y, top_left_x + 100))
                    .line_by((100, 0))
                    .close();
            }
            if grid[row][column][3]{
                data = data.move_to((top_left_y, top_left_x))
                    .line_by((0, 100))
                    .close();
            }
        }
    }
    
    //The lines up to here are just endpoints. This creates paths between the
    //endpoints with the desires settings.
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 6)
        .set("d", data);
    
    //Create the actual svg file and add the path we just created to it.
    let document = Document::new()
        .set("viewBox", (0, 0, svg_width, svg_height))
        .add(path);

    //Save the svg file.
    svg::save("maze.svg", &document).unwrap();

    //Tell user the file has been created.
    println!("Created SVG");
}

fn generate_with_recursive_randomized_depth_first(grid: &mut Vec<Vec<[bool; 5]>>) {
    
    //Recursively generate maze.
    rdf(grid, 0, 0);
}


fn rdf(grid: &mut Vec<Vec<[bool; 5]>>, cell_x: u32, cell_y: u32) {
    
    //Find the cell next to the current one that are unvisited.
    let mut adj_unvisited = count_adj_unvisited(grid, cell_x, cell_y);
    
    //Set this cell to visited.
    {
        let cell_x: usize = cell_x.try_into().unwrap();
        let cell_y: usize = cell_y.try_into().unwrap();
        grid[cell_x][cell_y][4] = true;
    }

    //Keep doing while there is no cells around this one.
    while adj_unvisited[4] {
        
        //Randomly select a surrounding cell.
        let mut selected_cell: usize = rand::thread_rng().gen_range(0..4);
        
        //Check if that cell was visited and keep randomly selecting an
        //unvisited cell is found.
        while adj_unvisited[selected_cell] {
            selected_cell = rand::thread_rng().gen_range(0..4);
        }

        //Get coords of next cell.
        let cell_x: i32 = cell_x.try_into().unwrap();
        let cell_y: i32 = cell_y.try_into().unwrap();
        let next_cell_x: u32 = (cell_x + REF_COORD[selected_cell][0]).try_into().unwrap();
        let next_cell_y: u32 = (cell_y + REF_COORD[selected_cell][1]).try_into().unwrap();

        //Open walls based on selection.
        {
            let cell_x: usize = cell_x.try_into().unwrap();
            let cell_y: usize = cell_y.try_into().unwrap();
            let next_cell_x: usize = next_cell_x.try_into().unwrap();
            let next_cell_y: usize = next_cell_y.try_into().unwrap();
            
            //Open wall in this cell.
            grid[cell_x][cell_y][selected_cell] = false;
            
            //Open wall in next cell.
            let next_selected_wall = (selected_cell + 2) % 4;
            grid[next_cell_x][next_cell_y][next_selected_wall] = false;
        }

        //Call rdf on next cell
        rdf(grid, next_cell_x, next_cell_y);

        //Check if cell still has placed next to it that are still unvisited.
        let cell_x: u32 = cell_x.try_into().unwrap();
        let cell_y: u32 = cell_y.try_into().unwrap();
        adj_unvisited = count_adj_unvisited(grid, cell_x, cell_y);
    }
}

fn count_adj_unvisited(grid: &mut Vec<Vec<[bool; 5]>>, cell_x: u32, cell_y: u32) -> [bool; 5] {
    
    //Convert cell coords into compatible types.
    let cell_x: usize = cell_x.try_into().unwrap();
    let cell_y: usize = cell_y.try_into().unwrap();
    
    //Start out by saying there are no adjacent unavailable spaces.
    let mut adj_unvisited = false;

    //Set the default of each surrounding cell to visited.
    let (mut cell_up, mut cell_right, mut cell_down, mut cell_left) 
        = (true, true, true, true);

    //The next if statements work similarly.
    //Check cell up.
    if cell_x > 0 {
        if grid[cell_x - 1][cell_y][4] == false{
            
            //If the cell is unvisited then there are adjacent unvisited cells
            //and this cell is has not been visited.
            adj_unvisited = true;
            cell_up = false;
        }
    }
    //Check cell right.
    if (cell_y + 1) < grid[1].len().try_into().unwrap() {
        if grid[cell_x][cell_y + 1][4] == false{
            adj_unvisited = true;
            cell_right = false;
        }
    }
    //Check cell down.
    if (cell_x + 1) < grid.len().try_into().unwrap() {
        if grid[cell_x + 1][cell_y][4] == false{
            adj_unvisited = true;
            cell_down = false;
        }
    }
    //Check cell left.
    if cell_y > 0 {
        if grid[cell_x][cell_y - 1][4] == false{
            adj_unvisited = true;
            cell_left = false;
        }
    }

    //Return all 5 booleans found.
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

    grid
}