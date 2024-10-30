use crossterm::{
    cursor, event::{Event, KeyCode, KeyEventKind}, execute, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, QueueableCommand
};
use std::{io::{stdout, Write}, vec};
use std::time::{Duration, Instant};

static FIELD_WIDTH: usize = 10;
static FIELD_HEIGHT: usize = 20;

static MESSAGE_DOWN: &str = "A - left, D - right, S/Down arrow - down"; 

static SHAPES: [[[i8; 4]; 4]; 7] = [[[0, 1, 1, 0],[0, 1, 1, 0],[0, 0, 0, 0],[0, 0, 0, 0]], [[0, 1, 0, 0],[0, 1, 0, 0],[0, 1, 0, 0],[0, 1, 0, 0]], [[0, 1, 1, 0],[1, 1, 0, 0],[0, 0, 0, 0],[0, 0, 0, 0]], [[1, 1, 0, 0],[0, 1, 1, 0],[0, 0, 0, 0],[0, 0, 0, 0]], [[0, 1, 0, 0],[0, 1, 0, 0],[0, 1, 1, 0],[0, 0, 0, 0]], [[0, 0, 1, 0],[0, 0, 1, 0],[0, 1, 1, 0],[0, 0, 0, 0]], [[1, 1, 1, 0],[0, 1, 0, 0],[0, 0, 0, 0],[0, 0, 0, 0]]];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(0, 0))?;

    let row: Vec<i8> = vec![0; FIELD_WIDTH];
    let mut grid: Vec<Vec<i8>>  = vec![row; FIELD_HEIGHT];

    // grid[3][3] = 2;
    grid[5][5] = 2;
    grid[5][6] = 2;
    grid[19] = vec![1; FIELD_WIDTH];
    grid[18][9] = 1;
    print_grid(grid.clone());
    stdout.execute(Print(MESSAGE_DOWN))?;

    let mut last_time = Instant::now();

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(last_time);

        if elapsed >= Duration::from_millis(10) {
            // clear all
            stdout.queue(Clear(ClearType::All))?;
            stdout.queue(cursor::MoveTo(0,0))?;

            // check available directions
            let mut av_dirs =  check_available_dirs(grid.clone());

            // edit row if needed
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
                                grid = spawn_shape(grid.clone(), 0);
                                break;
                            }
                        }
                    }

                    if key_event.code == KeyCode::Char('s') || key_event.code == KeyCode::Char('S') || key_event.code == KeyCode::Down{
                        // stdout.queue(Print("down\n"))?;
                        if av_dirs[3] == true {
                            grid = move_red(grid.clone(), 3);
                        }
                    }

                    if key_event.code == KeyCode::Left {
                        // stdout.queue(Print("arrow left\n"))?;
                    }
                    else if key_event.code == KeyCode::Right {
                        // stdout.queue(Print("arow right\n"))?;
                    }
                }           
            } //🔳⬜🟥

            print_grid(grid.clone());
            stdout.queue(Print(MESSAGE_DOWN))?;
            stdout.flush()?;

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }

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
