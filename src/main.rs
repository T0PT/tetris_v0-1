use crossterm::{
    cursor, event::{Event, KeyCode, KeyEventKind},
    execute, 
    style::Print, 
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, 
    ExecutableCommand, QueueableCommand
};
use std::{io::{stdout, Write}, vec};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(0, 0))?;

    let row: Vec<i8> = vec![0; 10];
    let mut grid: Vec<Vec<i8>>  = vec![row; 20];

    grid[3][3] = 2;
    print_grid(grid.clone());

    let mut last_time = Instant::now();

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(last_time);

        if elapsed >= Duration::from_millis(33) {
            // clear all
            stdout.queue(Clear(ClearType::All))?;
            stdout.queue(cursor::MoveTo(0,0))?;

            // edit row if needed
            let event = crossterm::event::read()?;
            // println!("Event: {:?}", event);
            if let Event::Key(key_event) = event {
                // println!("Key pressed: {:?}", key_event);
                if key_event.kind == KeyEventKind::Press {
                    if key_event.code == KeyCode::Char('a') || key_event.code == KeyCode::Char('A') {
                        // stdout.queue(Print("left\n"))?;
                        grid = move_red(grid.clone(), 2);
                    }
                    else if key_event.code == KeyCode::Char('d') || key_event.code == KeyCode::Char('D') {
                        // stdout.queue(Print("right\n"))?;
                        grid = move_red(grid.clone(), 0);
                    }

                    if key_event.code == KeyCode::Enter{
                        // stdout.queue(Print("enter\n"))?;
                    }

                    if key_event.code == KeyCode::Char('s') || key_event.code == KeyCode::Char('S') || key_event.code == KeyCode::Down{
                        // stdout.queue(Print("down\n"))?;
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

fn move_red(grid: Vec<Vec<i8>>, direction: i8) -> Vec<Vec<i8>> { //direction == 0: right, 1 - up, 2 - left, 3 - down
    let mut new_grid = vec![];
    if direction == 0 {
        for row in grid {
            let mut new_row = row.clone();
            for (index, value) in row.iter().enumerate() {
                if *value == 2 && index != 9 {
                    new_row[index + 1] = 2;
                    new_row[index] = 0;
                }
            }
            new_grid.push(new_row);
        }
    }
    else if direction == 2 {
        for row in grid {
            let mut new_row = row.clone();
            for (index, value) in row.iter().enumerate() {
                if *value == 2 && index != 0 {
                    new_row[index - 1] = 2;
                    new_row[index] = 0;
                }
            }
            new_grid.push(new_row);
        }
    }
    return new_grid;
}
