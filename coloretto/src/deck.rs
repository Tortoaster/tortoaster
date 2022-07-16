use std::iter;

use rand::seq::SliceRandom;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn small() -> Self {
        let mut cards = Card::COLORS;
        cards.shuffle(&mut rand::thread_rng());
        Deck::generate_with(&cards[2..])
    }

    pub fn medium() -> Self {
        let mut cards = Card::COLORS;
        cards.shuffle(&mut rand::thread_rng());
        Deck::generate_with(&cards[1..])
    }

    pub fn large() -> Self {
        Deck::generate_with(&Card::COLORS[..])
    }

    fn generate_with(colors: &[Card]) -> Deck {
        let mut cards: Vec<_> = colors
            .iter()
            .chain(Card::OTHER.iter())
            .flat_map(|card| iter::repeat(*card).take(card.amount()))
            .collect();
        cards.shuffle(&mut rand::thread_rng());

        Deck { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Card {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Brown,
    Rainbow,
    Gold,
    Points,
}

impl Card {
    const COLORS: [Card; 7] = [
        Card::Red,
        Card::Orange,
        Card::Yellow,
        Card::Green,
        Card::Blue,
        Card::Purple,
        Card::Brown,
    ];
    const OTHER: [Card; 3] = [Card::Rainbow, Card::Gold, Card::Points];

    const fn amount(&self) -> usize {
        match self {
            Card::Red
            | Card::Orange
            | Card::Yellow
            | Card::Green
            | Card::Blue
            | Card::Purple
            | Card::Brown => 9,
            Card::Rainbow => 2,
            Card::Gold => 1,
            Card::Points => 10,
        }
    }
}
