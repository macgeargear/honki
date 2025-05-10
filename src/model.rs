#[derive(Debug, Clone)]
pub struct Flashcard {
    pub front: String,
    pub back: String,
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub name: String,
    pub flashcards: Vec<Flashcard>,
}

#[derive(PartialEq)]
pub enum Screen {
    DeckSelection,
    Flashcards,
}

pub struct Model {
    pub decks: Vec<Deck>,
    pub current_deck_index: usize,
    pub current_card_index: usize,
    pub show_front: bool,
    pub screen: Screen,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            decks: vec![
                Deck {
                    name: "Rust Basics".into(),
                    flashcards: vec![
                        Flashcard {
                            front: "What is ownership?".into(),
                            back: "Memory management model in Rust".into(),
                        },
                        Flashcard {
                            front: "What is borrowing?".into(),
                            back: "Accessing data without taking ownership".into(),
                        },
                    ],
                },
                Deck {
                    name: "Capital Cities".into(),
                    flashcards: vec![Flashcard {
                        front: "Capital of France?".into(),
                        back: "Paris".into(),
                    }],
                },
            ],
            current_card_index: 0,
            current_deck_index: 0,
            show_front: true,
            screen: Screen::DeckSelection,
        }
    }
}

impl Model {
    pub fn current_deck(&self) -> &Deck {
        &self.decks[self.current_deck_index]
    }
}
