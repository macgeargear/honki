use crate::model::{Model, Screen};
use crate::msg::Msg;

pub fn update(msg: Msg, mut model: Model) -> Model {
    match msg {
        Msg::Quit => std::process::exit(0),
        Msg::NextCard => {
            if model.current_card_index + 1 < model.current_deck().flashcards.len() {
                model.current_card_index += 1;
                model.show_front = true;
            }
        }
        Msg::PrevCard => {
            if model.current_card_index > 0 {
                model.current_card_index -= 1;
                model.show_front = true;
            }
        }
        Msg::FlipCard => model.show_front = !model.show_front,
        Msg::NavigateDeckUp => {
            if model.current_deck_index > 0 {
                model.current_deck_index -= 1;
            }
        }
        Msg::NavigateDeckDown => {
            if model.current_deck_index + 1 < model.decks.len() {
                model.current_deck_index += 1;
            }
        }
        Msg::ConfirmDeck => {
            model.screen = Screen::Flashcards;
            model.current_card_index = 0;
            model.show_front = true;
        }
        Msg::SelectDeck(i) => model.current_deck_index = i,
        Msg::BacktoSelectDeck => model.screen = Screen::DeckSelection,
    }
    model
}
