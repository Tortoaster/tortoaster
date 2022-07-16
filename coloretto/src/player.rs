use crate::{Card, Error, Stack};

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Player {
    cards: Vec<Card>,
    stack: Option<Stack>,
}

impl Player {
    pub fn new() -> Self {
        Player::default()
    }

    pub fn insert_stack(&mut self, stack: Stack) {
        self.stack = Some(stack);
    }

    pub fn collect_stack(&mut self) -> Result<Stack, Error> {
        self.cards
            .append(self.stack.as_mut().ok_or(Error::NoStack)?.get_cards_mut());
        Ok(self.stack.take().unwrap())
    }

    pub fn has_stack(&self) -> bool {
        self.stack.is_some()
    }
}
