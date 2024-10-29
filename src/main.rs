use crossterm::{
    cursor, event::{Event, KeyCode, KeyEventKind}, execute, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, QueueableCommand
};
use std::io::{stdout, Write};

 fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(0, 0))?;

    let row: Vec<i8> = vec![0; 10];
    let mut grid: Vec<Vec<i8>>  = vec![row; 20];

    grid[3][3] = 2;
    print_grid(grid.clone());

    loop {
        let event = crossterm::event::read()?;
        // println!("Event: {:?}", event);
        if let Event::Key(key_event) = event {
            // println!("Key pressed: {:?}", key_event);
            if key_event.kind == KeyEventKind::Press {
                if key_event.code == KeyCode::Char('a') || key_event.code == KeyCode::Char('A') {
                    stdout.queue(Print("left\n"))?;
                }
                else if key_event.code == KeyCode::Char('d') || key_event.code == KeyCode::Char('D') {
                    stdout.queue(Print("right\n"))?;
                }

                if key_event.code == KeyCode::Enter{
                    stdout.queue(Print("enter\n"))?;
                }

                if key_event.code == KeyCode::Char('s') || key_event.code == KeyCode::Char('S') || key_event.code == KeyCode::Down{
                    stdout.queue(Print("down\n"))?;
                }

                if key_event.code == KeyCode::Left {
                    stdout.queue(Print("arrow left\n"))?;
                }
                else if key_event.code == KeyCode::Right {
                    stdout.queue(Print("arow right\n"))?;
                }
                if key_event.code == KeyCode::Backspace {
                    stdout.queue(Clear(ClearType::All))?;
                    stdout.queue(cursor::MoveTo(0,0))?;
                    print_grid(grid.clone());
                }
            }           
        } //🔳⬜🟥
        stdout.flush()?;

        if event == Event::Key(KeyCode::Esc.into()) {
            break;
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