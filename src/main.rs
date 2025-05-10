mod model;
mod msg;
mod update;
mod view;

use std::io::stdout;

use color_eyre::eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use model::{Model, Screen};
use msg::Msg;
use ratatui::{Terminal, prelude::CrosstermBackend};
use update::update;
use view::view;

fn main() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut model = Model::default();

    loop {
        terminal.draw(|f| view(&model, f))?;

        if let Event::Key(key) = event::read()? {
            let msg = match key.code {
                KeyCode::Char('q') => Msg::Quit,
                KeyCode::Char('b') => Msg::BacktoSelectDeck,
                KeyCode::Char(' ') => Msg::FlipCard,
                KeyCode::Left => Msg::PrevCard,
                KeyCode::Right => Msg::NextCard,
                KeyCode::Up => match model.screen {
                    Screen::DeckSelection => Msg::NavigateDeckUp,
                    _ => continue,
                },
                KeyCode::Down => match model.screen {
                    Screen::DeckSelection => Msg::NavigateDeckDown,
                    _ => continue,
                },
                KeyCode::Enter => Msg::ConfirmDeck,
                _ => continue,
            };

            model = update(msg, model);
        }
    }
}
