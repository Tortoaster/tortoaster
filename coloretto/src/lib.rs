use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use crate::action::Action;
use crate::deck::{Card, Deck};
use crate::player::Player;
use crate::stack::Stack;

mod action;
mod deck;
mod player;
mod stack;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Coloretto {
    players: Vec<Player>,
    deck: Deck,
    stacks: Vec<Stack>,
    current: Option<Card>,
    turn: usize,
    winner: Option<usize>,
}

impl Coloretto {
    pub fn new(size: usize) -> Result<Self, Error> {
        if size < 2 || size > 5 {
            return Err(Error::Players);
        }

        let players: Vec<_> = (0..size).map(|_| Player::new()).collect();
        let deck = if size == 2 {
            Deck::small()
        } else if size == 3 {
            Deck::medium()
        } else {
            Deck::large()
        };
        let stacks = if size == 2 {
            vec![Stack::new(1), Stack::new(2), Stack::new(3)]
        } else {
            (0..size).map(|_| Stack::new(3)).collect()
        };

        Ok(Coloretto {
            players,
            deck,
            stacks,
            current: None,
            turn: 0,
            winner: None,
        })
    }

    pub fn perform(&mut self, action: Action) -> Result<(), Error> {
        if self.winner.is_some() {
            return Err(Error::GameOver);
        }

        if self.current.is_none() {
            match action {
                Action::Flip => {
                    self.current = self.deck.draw();
                    Ok(())
                }
                Action::Place(_) => Err(Error::NoCard),
                Action::Take(index) => {
                    let stack = self.stacks.get(index).ok_or(Error::NoStack)?;
                    if stack.is_empty() {
                        Err(Error::EmptyStack)
                    } else {
                        let mut stack = self.stacks.remove(index);
                        if stack.has_gold() {
                            stack.push(self.deck.draw().unwrap());
                        }
                        self.players[self.turn].insert_stack(stack);
                        if self.players.iter().all(Player::has_stack) {
                            self.stacks.iter_mut().for_each(|stack| stack.clear());
                            let mut stacks = self
                                .players
                                .iter_mut()
                                .map(|player| player.collect_stack().unwrap())
                                .collect();
                            self.stacks.append(&mut stacks);
                            self.stacks.sort();
                        } else {
                            self.turn = (self.turn + 1) % self.players.len();
                        }
                        Ok(())
                    }
                }
            }
        } else {
            match action {
                Action::Flip => Err(Error::Flipped),
                Action::Place(index) => {
                    let stack = self.stacks.get_mut(index).ok_or(Error::NoStack)?;
                    if !stack.is_full() {
                        stack.push(self.current.take().unwrap());
                        self.turn = (self.turn + 1) % self.players.len();
                        Ok(())
                    } else {
                        Err(Error::FullStack)
                    }
                }
                Action::Take(_) => Err(Error::Flipped),
            }
        }
    }

    pub fn turn(&self) -> usize {
        self.turn
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Error)]
pub enum Error {
    #[error("invalid number of players")]
    Players,
    #[error("there is no card to place")]
    NoCard,
    #[error("you already took a card")]
    Flipped,
    #[error("that stack doesn't exist")]
    NoStack,
    #[error("you cannot take an empty stack")]
    EmptyStack,
    #[error("that stack is full")]
    FullStack,
    #[error("the game is already over")]
    GameOver,
}
