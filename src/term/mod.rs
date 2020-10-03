// Example adapted from:
// https://github.com/Canop/termimad/blob/master/examples/scrollable/main.rs

use crossterm::{
    cursor::{ Hide, Show},
    event::{
        self,
        Event,
        KeyEvent,
        KeyCode::*,
    },
    queue,
    terminal::{
        self,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    style::Color::*,
};
use std::io::{stderr, Write};
use termimad::*;

pub fn run(markdown: String) -> Result<()>{
    let skin = make_skin();
    run_app(skin, &markdown)
}

pub fn clear() -> Result<()> {
    let mut w = stderr(); // we could also have used stdout
    queue!(w, Clear(ClearType::All))?;
    Ok(())
}

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120); // we don't want a too wide text column
    area
}

fn run_app(skin: MadSkin, markdown: &str) -> Result<()> {
    let mut w = stderr(); // we could also have used stdout
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?; // hiding the cursor
    let mut view = MadView::from(markdown.to_owned(), view_area(), skin);
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent{code, ..})) => {
                match code {
                    Up | Char('k') => view.try_scroll_lines(-1),
                    Down | Char('j') => view.try_scroll_lines(1),
                    PageUp => view.try_scroll_pages(-1),
                    PageDown => view.try_scroll_pages(1),
                    Char('q') => break,
                    _ => continue,
                }
            }
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
}

