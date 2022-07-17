use serde::{Deserialize, Serialize};

use crate::Card;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Stack {
    cards: Vec<Card>,
    capacity: usize,
}

impl Stack {
    pub fn new(capacity: usize) -> Self {
        Stack {
            cards: Vec::new(),
            capacity,
        }
    }

    pub fn get_cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.cards.len() >= self.capacity
    }

    pub fn has_gold(&self) -> bool {
        self.cards.contains(&Card::Gold)
    }
}
