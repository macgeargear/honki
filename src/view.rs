use crate::model::{Model, Screen};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};

pub fn view(model: &Model, f: &mut Frame) {
    match model.screen {
        Screen::DeckSelection => view_deck_selection(model, f),
        Screen::Flashcards => view_flashcards(model, f),
    }
}

fn view_deck_selection(model: &Model, f: &mut Frame) {
    let area = f.area();
    let items: Vec<ListItem> = model
        .decks
        .iter()
        .map(|d| ListItem::new(d.name.clone()))
        .collect();
    let list = List::new(items)
        .block(
            Block::default()
                .title("Select a Deck")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol("▶ ");
    let mut state = ListState::default();
    state.select(Some(model.current_deck_index));
    f.render_stateful_widget(list, area, &mut state);
}

fn view_flashcards(model: &Model, f: &mut Frame) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(area);

    let card = &model.current_deck().flashcards[model.current_card_index];
    let content = if model.show_front {
        &card.front
    } else {
        &card.back
    };

    let card_block = Paragraph::new(content.clone())
        .block(
            Block::default()
                .title(format!(
                    "Flashcard {}/{}",
                    model.current_card_index + 1,
                    model.current_deck().flashcards.len()
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

    let footer = Paragraph::new("←/→: Navigate | Space: Flip | B: Select Deck | Q: Quit")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));

    f.render_widget(card_block, chunks[0]);
    f.render_widget(footer, chunks[1]);
}
