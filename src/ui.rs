use crate::model::{Deck, FlashCard};
use color_eyre::eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io::stdout;

pub fn select_dect<'a>(decks: &'a [Deck]) -> Result<&'a Deck> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut index = 0;

    loop {
        terminal.draw(|f| {
            let area = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
                .split(area);

            let items: Vec<ListItem> = decks
                .iter()
                .map(|d| ListItem::new(d.name.clone()))
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Select Deck").borders(Borders::ALL))
                .highlight_style(Style::default().fg(Color::Yellow))
                .highlight_symbol(">> ");

            f.render_stateful_widget(
                list,
                chunks[0],
                &mut ratatui::widgets::ListState::default().with_selected(Some(index)),
            );

            let instructions = Paragraph::new("↑/↓: Navigate | Enter: Select | Q: Quit")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray));

            f.render_widget(instructions, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    cleanup_terminal(&mut terminal)?;
                    std::process::exit(0);
                }
                KeyCode::Down => {
                    if index + 1 < decks.len() {
                        index += 1;
                    }
                }
                KeyCode::Up => {
                    if index > 0 {
                        index -= 1;
                    }
                }
                KeyCode::Enter => {
                    cleanup_terminal(&mut terminal)?;
                    return Ok(&decks[index]);
                }
                _ => {}
            }
        }
    }
}

pub fn run_app(deck: &Deck) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut index = 0;
    let mut show_front = true;

    loop {
        render_flashcard(&mut terminal, &deck.flashcards, index, show_front)?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Left => {
                    if index > 0 {
                        index -= 1;
                        show_front = true;
                    }
                }
                KeyCode::Right => {
                    if index + 1 < deck.flashcards.len() {
                        index += 1;
                        show_front = true;
                    }
                }
                KeyCode::Char(' ') => {
                    show_front = !show_front;
                }
                _ => {}
            }
        }
    }

    cleanup_terminal(&mut terminal)?;
    Ok(())
}

fn render_flashcard(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    flashcards: &[FlashCard],
    index: usize,
    show_front: bool,
) -> Result<()> {
    let content = if show_front {
        &flashcards[index].front
    } else {
        &flashcards[index].back
    };

    terminal.draw(|f| {
        let area = f.area();

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(area);

        let footer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[1]);

        let block = Block::default()
            .title(format!("Flashcard {}/{}", index + 1, flashcards.len()))
            .borders(Borders::ALL);

        let paragraph = Paragraph::new(content.clone())
            .block(block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));

        f.render_widget(paragraph, main_layout[0]);

        let instructions = Paragraph::new("Space: Flip card | Left/Right: Navigate | Q: Quit")
            .alignment(Alignment::Center);

        let next_vocab_actions =
            Paragraph::new("1: Again(<1m) | 2: Hard(<6m) | 3: Good(<1m) | 4: Easy(3d)")
                .alignment(Alignment::Center);

        f.render_widget(next_vocab_actions, footer_layout[0]);
        f.render_widget(instructions, footer_layout[1]);
    })?;

    Ok(())
}

fn cleanup_terminal(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
