use crossterm::{
    cursor, event::{Event, KeyCode, KeyEventKind, poll}, execute, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, QueueableCommand
};
use core::error;
use std::{io::{stdout, Write}, vec};
use std::time::{Duration, Instant};
use rand::Rng;

static FIELD_WIDTH: usize = 10;
static FIELD_HEIGHT: usize = 20;
static TIME_OF_FRAME: u64 = 10; // in millis
static FRAMES_BETWEEN_DOWN: i16 = 100;

static MESSAGE_DOWN: &str = "A - left, D - right\nS/Down arrow - down\nESC - exit";

static SHAPES: [[[i8; 4]; 4]; 7] = [[[0, 1, 1, 0],[0, 1, 1, 0],[0, 0, 0, 0],[0, 0, 0, 0]], [[0, 1, 0, 0],[0, 1, 0, 0],[0, 1, 0, 0],[0, 1, 0, 0]], [[0, 1, 1, 0],[1, 1, 0, 0],[0, 0, 0, 0],[0, 0, 0, 0]], [[1, 1, 0, 0],[0, 1, 1, 0],[0, 0, 0, 0],[0, 0, 0, 0]], [[0, 1, 0, 0],[0, 1, 0, 0],[0, 1, 1, 0],[0, 0, 0, 0]], [[0, 0, 1, 0],[0, 0, 1, 0],[0, 1, 1, 0],[0, 0, 0, 0]], [[1, 1, 1, 0],[0, 1, 0, 0],[0, 0, 0, 0],[0, 0, 0, 0]]];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(0, 0))?;

    let mut rng = rand::thread_rng();

    let row: Vec<i8> = vec![0; FIELD_WIDTH];
    let mut grid: Vec<Vec<i8>>  = vec![row; FIELD_HEIGHT];

    // grid[3][3] = 2;
    // grid[5][5] = 2;
    // grid[5][6] = 2;
    // grid[19] = vec![1; FIELD_WIDTH];
    // grid[18][9] = 1;
    grid = spawn_shape(grid, rng.gen_range(0..7));
    print_grid(grid.clone());
    stdout.execute(Print(MESSAGE_DOWN))?;

    let mut last_time = Instant::now();
    let mut last_move_elapsed: i16 = 0;

    let mut error_message: String = format!("");

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(last_time);

        if elapsed >= Duration::from_millis(TIME_OF_FRAME) {
            // add counter
            last_move_elapsed += 1;

            // clear all
            stdout.queue(Clear(ClearType::All))?;
            stdout.queue(cursor::MoveTo(0,0))?;

            // check available directions
            let mut av_dirs =  check_available_dirs(grid.clone());

            // check if an event is available if not - pass
            let available: bool = poll(Duration::from_secs(0))?;
            if available == true {
                let event = crossterm::event::read()?;
                // println!("Event: {:?}", event);
                if let Event::Key(key_event) = event {
                    // println!("Key pressed: {:?}", key_event);
                    if key_event.kind == KeyEventKind::Press {
                        if key_event.code == KeyCode::Char('a') || key_event.code == KeyCode::Char('A') {
                            // stdout.queue(Print("left\n"))?;
                            if av_dirs[2] == true {
                                grid = move_red(grid.clone(), 2);
                            }
                        }
                        else if key_event.code == KeyCode::Char('d') || key_event.code == KeyCode::Char('D') {
                            // stdout.queue(Print("right\n"))?;
                            if av_dirs[0] == true {
                                grid = move_red(grid.clone(), 0);
                            }
                        }

                        if key_event.code == KeyCode::Enter{
                            // stdout.queue(Print("enter\n"))?;
                            // grid = spawn_shape(grid.clone(), 0);
                            loop {
                                if av_dirs[3] == true {
                                    grid = move_red(grid.clone(), 3);
                                    av_dirs = check_available_dirs(grid.clone());
                                }
                                else {
                                    grid = red_to_white(grid.clone());
                                    grid = spawn_shape(grid.clone(), rng.gen_range(0..7));
                                    last_move_elapsed = 0;
                                    break;
                                }
                            }
                        }

                        if key_event.code == KeyCode::Char('s') || key_event.code == KeyCode::Char('S') || key_event.code == KeyCode::Down{
                            // stdout.queue(Print("down\n"))?;
                            if av_dirs[3] == true {
                                grid = move_red(grid.clone(), 3);
                                last_move_elapsed = 0;
                            }
                            else {
                                grid = red_to_white(grid.clone());
                                grid = spawn_shape(grid.clone(), 0);
                                last_move_elapsed = 0;
                            }
                        }

                        if key_event.code == KeyCode::Left {
                            // stdout.queue(Print("arrow left\n"))?;
                            // grid = rotate_red(grid.clone(), 3);
                        }
                        else if key_event.code == KeyCode::Right {
                            // stdout.queue(Print("arow right\n"))?;
                            grid = rotate_red(grid.clone(), 1);
                        }
                    }           
                } //🔳⬜🟥
                if event == Event::Key(KeyCode::Esc.into()) {
                    break;
                }
    
            }

            // error_message = format!("\n\n{}", last_move_elapsed);
            // move if wait time between moves gets too long - move down
            if last_move_elapsed >= FRAMES_BETWEEN_DOWN - 1 {
                if av_dirs[3] == true {
                    grid = move_red(grid.clone(), 3);
                    last_move_elapsed = 0;
                }
                else {
                    grid = red_to_white(grid.clone());
                    grid = spawn_shape(grid.clone(), rng.gen_range(0..7));
                }
            }

            print_grid(grid.clone());
            stdout.queue(Print(MESSAGE_DOWN))?;
            stdout.queue(Print(&error_message))?;
            stdout.flush()?;

            last_time = now;
        }        
    }
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn print_grid(grid: Vec<Vec<i8>>) {
    for row in grid {
        for cell in row {
            if cell == 0 {
                print!("🔳");
            }
            else if cell == 1{
                print!("⬜");
            }
            else if cell == 2{
                print!("🟥");
            }
        }
        println!();
    }
}

fn rotate_red(grid: Vec<Vec<i8>>, times: i8) -> Vec<Vec<i8>> { // times = 1 - 90 degrees clockwise, 2 - 180 degrees, 3 - 270 clockwise
    let mut final_grid: Vec<Vec<i8>> = grid.clone();
    for i in 0..times {
        let mut new_grid: Vec<Vec<i8>> = final_grid.clone();
        let mut red_cells: [usize; 4]; red_cells = [127, 127, 0, 0]; // [y1, x1, y2, x2]
        // find where are the red cells
        for (index_y, row) in grid.iter().enumerate() {
            for (index_x, value) in row.iter().enumerate() {
                if *value == 2 {
                    if index_y < red_cells[0] {
                        red_cells[0] = index_y;
                    }
                    else if index_y > red_cells[2] {
                        red_cells[2] = index_y;
                    }
                    if index_x < red_cells[1] {
                        red_cells[1] = index_x;
                    }
                    else if index_x > red_cells[3] {
                        red_cells[3] = index_x;
                    }
                    new_grid[index_y][index_x] = 0;
                }
            }
        }
        // transpose the matrix
        let len_x = red_cells[3] - red_cells[1] + 1;
        let len_y = red_cells[2] - red_cells[0] + 1;
        for y in 0..len_x {
            for x in 0..len_y {
                new_grid[red_cells[0] + y][red_cells[1] + x] = grid[red_cells[0] + x][red_cells[1] + y];                
            }
        }
        // reverse each row where red
        let mut new_new_grid: Vec<Vec<i8>> = new_grid.clone();
        for y in 0..len_x {
            for x in 0..len_y + 1 {
                new_new_grid[red_cells[0] + y][red_cells[1] + x] = new_grid[red_cells[0] + y][red_cells[1] + len_y - x];
            }
        }
        final_grid = new_new_grid;
    }
    return final_grid;
}

fn red_to_white(grid: Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let mut new_grid = grid.clone();
    for (index_y, row) in grid.iter().enumerate() {
        for (index_x, value) in row.iter().enumerate() {
            if *value == 2 {
                new_grid[index_y][index_x] = 1;
            }
        }
    }
    return new_grid;
}

fn spawn_shape(grid: Vec<Vec<i8>>, shape: i8) -> Vec<Vec<i8>> {
    let mut new_grid = grid.clone();
    let to_add = SHAPES[shape as usize].clone();
    for (index_y, row) in grid.iter().enumerate() {
        for (index_x, value) in row.iter().enumerate() {
            if index_y < 4 {
                if index_x > 2 && index_x < 7 {
                    if *value != 0 && to_add[index_y][index_x - 3] == 1 {
                        return grid
                    }
                    else if to_add[index_y][index_x - 3] == 1 {
                        new_grid[index_y][index_x] = 2
                    }
                }
            }
        }
    }
    return new_grid;
}

fn check_available_dirs(grid: Vec<Vec<i8>>) -> [bool; 4] {
    let mut available_dirs: [bool; 4] = [true; 4];
    for (index_y, row) in grid.iter().enumerate() {
        for (index_x, value) in row.iter().enumerate() {
            if *value == 2 {
                if index_x == 0 || row[index_x - 1] == 1 {
                    available_dirs[2] = false;
                }
                else if index_x == FIELD_WIDTH - 1 || row[index_x + 1] == 1 {
                    available_dirs[0] = false;
                }
                if index_y == 0 || grid[index_y - 1][index_x] == 1 {
                    available_dirs[1] = false;
                }
                else if index_y == FIELD_HEIGHT - 1 || grid[index_y + 1][index_x] == 1{
                    available_dirs[3] = false;
                } 
            }
        }
    }
    return available_dirs
}

fn move_red(grid: Vec<Vec<i8>>, direction: i8) -> Vec<Vec<i8>> { //direction == 0 - right, 1 - up, 2 - left, 3 - down
    let mut new_grid = vec![];
    if direction == 0 {
        for row in grid {
            let mut new_row = vec![0; row.len()]; // new row is just zeros
            for (index, value) in row.iter().enumerate() {
                if *value == 2 && index != 9 {
                    new_row[index + 1] = 2;
                }
                else if *value == 1 {
                    new_row[index] = 1;
                }
            }
            new_grid.push(new_row);
        }
    }
    else if direction == 2 {
        for row in grid {
            let mut new_row = vec![0; row.len()]; // new row is just zeros
            for (index, value) in row.iter().enumerate() {
                if *value == 2 && index != 0 {
                    new_row[index - 1] = 2;
                }
                else if *value == 1 {
                    new_row[index] = 1;
                }
            }
            new_grid.push(new_row);
        }
    }
    else if direction == 3 {
        new_grid = vec![vec![0; FIELD_WIDTH]; FIELD_HEIGHT];
        for (index_y, row) in grid.iter().enumerate() {
            for (index_x, value) in row.iter().enumerate() {
                if *value == 2 {
                    new_grid[index_y + 1][index_x] = 2;
                }
                else if *value == 1 {
                    new_grid[index_y][index_x] = 1;
                } 
            }
        }
    }
    else if direction == 1 {
        new_grid = vec![vec![0; FIELD_WIDTH]; FIELD_HEIGHT];
        for (index_y, row) in grid.iter().enumerate() {
            for (index_x, value) in row.iter().enumerate() {
                if *value == 2 {
                    new_grid[index_y - 1][index_x] = 2;
                }
                else if *value == 1 {
                    new_grid[index_y][index_x] = 1;
                } 
            }
        }
    }
    return new_grid;
}
