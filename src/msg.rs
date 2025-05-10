pub enum Msg {
    Quit,
    NextCard,
    PrevCard,
    FlipCard,
    SelectDeck(usize),
    NavigateDeckUp,
    NavigateDeckDown,
    ConfirmDeck,
    BacktoSelectDeck,
}
