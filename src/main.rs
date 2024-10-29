use crossterm::{
    event::{Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout};

 fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    loop {
        let event = crossterm::event::read()?;
        // println!("Event: {:?}", event);
        if let Event::Key(key_event) = event {
            // println!("Key pressed: {:?}", key_event);
            if key_event.kind == KeyEventKind::Press {
                if key_event.code == KeyCode::Char('a') || key_event.code == KeyCode::Char('A') {
                    println!("left");
                }
                else if key_event.code == KeyCode::Char('d') || key_event.code == KeyCode::Char('D') {
                    println!("right");
                }

                if key_event.code == KeyCode::Enter{
                    println!("enter");
                }

                if key_event.code == KeyCode::Char('s') || key_event.code == KeyCode::Char('S') || key_event.code == KeyCode::Down{
                    println!("down");
                }

                if key_event.code == KeyCode::Left {
                    println!("arrow left");
                }
                else if key_event.code == KeyCode::Right {
                    println!("arrow right");
                }
            }
        } //🔳⬜

        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}