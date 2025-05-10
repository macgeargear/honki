pub struct FlashCard {
    pub front: String,
    pub back: String,
}

pub struct Deck {
    pub name: String,
    pub flashcards: Vec<FlashCard>,
}
