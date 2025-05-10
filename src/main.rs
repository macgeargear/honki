mod model;
mod ui;

use crate::model::{Deck, FlashCard};

use color_eyre::eyre::Result;
use ui::{run_app, select_dect};

fn main() -> Result<()> {
    color_eyre::install()?;

    let decks = vec![
        Deck {
            name: "General Knowledge".into(),
            flashcards: vec![
                FlashCard {
                    front: "What is the capital of France?".into(),
                    back: "Paris".into(),
                },
                FlashCard {
                    front: "What does 'ownership' mean in Rust?".into(),
                    back: "It means a variable owns memory and is responsible for freeing it."
                        .into(),
                },
            ],
        },
        Deck {
            name: "Rust Syntax".into(),
            flashcards: vec![FlashCard {
                front: "What is a slice?".into(),
                back: "&[T] — a view into a sequence".into(),
            }],
        },
    ];

    let selected_deck = select_dect(&decks)?;
    run_app(selected_deck)
}
