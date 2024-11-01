use crossterm::{
    cursor, event::{Event, KeyCode, KeyEventKind, poll}, execute, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, QueueableCommand
};
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
    let mut score: i32 = 0;
    let mut game_over: bool = false;

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(last_time);

        if elapsed >= Duration::from_millis(TIME_OF_FRAME) {
            // add counter
            last_move_elapsed += 1;


            // check available directions
            let mut av_dirs =  check_available_dirs(grid.clone());

            // check if an event is available if not - pass
            let available: bool = poll(Duration::from_secs(0))?;
            if available == true {
                let event = crossterm::event::read()?;
                // println!("Event: {:?}", event);
                if let Event::Key(key_event) = event {
                    if game_over == false {
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
                            loop {
                                if av_dirs[3] == true {
                                    grid = move_red(grid.clone(), 3);
                                    av_dirs = check_available_dirs(grid.clone());
                                }
                                else {
                                    grid = red_to_white(grid.clone());
                                    let new_grid = spawn_shape(grid.clone(), rng.gen_range(0..7));
                                    if new_grid == grid {
                                        game_over = true;
                                    }
                                    else {
                                        grid = new_grid;
                                    }
                                    score += 1;
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
                                let new_grid = spawn_shape(grid.clone(), rng.gen_range(0..7));
                                if new_grid == grid {
                                    game_over = true;
                                }
                                else {
                                    grid = new_grid;
                                }
                                score += 1;
                                last_move_elapsed = 0;
                            }
                        }

                        if key_event.code == KeyCode::Left {
                            // stdout.queue(Print("arrow left\n"))?;
                            for _ in 0..3 {
                                grid = rotate_red(grid.clone());
                            }
                        }
                        else if key_event.code == KeyCode::Right {
                            // stdout.queue(Print("arow right\n"))?;
                            grid = rotate_red(grid.clone());
                        }
                    }           
                    } //🔳⬜🟥
                }
                    
                if event == Event::Key(KeyCode::Esc.into()) {
                    break;
                }
    
            }

            // error_message = format!("\n\n{}", last_move_elapsed);
            // move if wait time between moves gets too long - move down
            if game_over == false {
                if last_move_elapsed >= FRAMES_BETWEEN_DOWN - 1 {
                    if av_dirs[3] == true {
                        grid = move_red(grid.clone(), 3);
                        last_move_elapsed = 0;
                    }
                    else {
                        grid = red_to_white(grid.clone());
                        let new_grid = spawn_shape(grid.clone(), rng.gen_range(0..7));
                        if new_grid == grid {
                            game_over = true;
                        }
                        else {
                            grid = new_grid;
                        }
                        score += 1;
                    }
                }
            }
            

            // clean all white lines
            grid = clean_white_lines(grid.clone());

            // clear all
            stdout.queue(Clear(ClearType::All))?;
            stdout.queue(cursor::MoveTo(0,0))?;
            print_grid(grid.clone());
            stdout.queue(Print(MESSAGE_DOWN))?;
            stdout.queue(Print(format!("\n\nSCORE: {}", score)))?;
            // error_message = format!("\n\n{}", game_over);
            stdout.queue(Print(&error_message))?;
            if game_over {
                stdout.queue(Print("\n\n   ___    _    __  __  ___    ___ __   __ ___  ___ \n  / __|  /_\\  |  \\/  || __|  / _ \\\\ \\ / /| __|| _ \\\n | (_ | / _ \\ | |\\/| || _|  | (_) |\\ V / | _| |   /\n  \\___|/_/ \\_\\|_|  |_||___|  \\___/  \\_/  |___||_|_\\\n"))?;              
            }
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

fn clean_white_lines(grid: Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let mut new_grid: Vec<Vec<i8>> = grid.clone();
    let mut lines_to_clean: Vec<usize> = vec![];
    for (index_y, row) in grid.iter().enumerate() {
        let mut is_white: bool = true;
        for value in row {
            if *value == 0 {
                is_white = false;
                break;
            }
        }
        if is_white == true {
            lines_to_clean.push(index_y);
        }
    }
    for line in lines_to_clean {
        new_grid.remove(line);
        new_grid.insert(0, vec![0; FIELD_WIDTH]);
    }
    return new_grid;
}

fn rotate_red(grid: Vec<Vec<i8>>) -> Vec<Vec<i8>> { // 90 degrees clockwise
    let mut new_grid: Vec<Vec<i8>> = grid.clone();
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
    
    // if its too wide - return the original grid
    if len_y + red_cells[1] >= FIELD_WIDTH || len_x + red_cells[0] >= FIELD_HEIGHT {
        return grid;
    }

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
    new_new_grid = move_red(new_new_grid.clone(), 2);
    return new_new_grid;    
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
