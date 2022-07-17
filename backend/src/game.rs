use std::fmt::{Display, Formatter};
use std::str::FromStr;

use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use thiserror::Error;

use coloretto::{Action, Coloretto};

use crate::User;

#[derive(Copy, Clone, Debug, SerializeDisplay, DeserializeFromStr)]
pub struct RoomId([u8; 4]);

impl RoomId {
    fn generate() -> Self {
        let id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        RoomId(id)
    }
}

impl Display for RoomId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.0[0] as char, self.0[1] as char, self.0[2] as char, self.0[3] as char
        )
    }
}

impl FromStr for RoomId {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 4 {
            let id = s
                .chars()
                .map(u8::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| GameError::Invalid)?
                .try_into()
                .unwrap();
            Ok(RoomId(id))
        } else {
            Err(GameError::Invalid)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    id: RoomId,
    players: Vec<User>,
    status: Status,
}

impl Room {
    pub fn new(creator: User) -> Self {
        Room {
            id: RoomId::generate(),
            players: vec![creator],
            status: Status::Waiting,
        }
    }

    pub fn id(&self) -> RoomId {
        self.id
    }

    pub fn enroll(&mut self, user: User) -> Result<(), GameError> {
        match self.status {
            Status::Waiting => {
                if !self.players.contains(&user) {
                    if self.players.len() < 5 {
                        self.players.push(user);
                        Ok(())
                    } else {
                        Err(GameError::Full)
                    }
                } else {
                    Err(GameError::AlreadyJoined)
                }
            }
            Status::Playing(_) => Err(GameError::WrongState),
        }
    }

    pub fn remove(&mut self, user: &User) -> Result<(), GameError> {
        match self.status {
            Status::Waiting => Ok(self.players.retain(|u| *u != *user)),
            Status::Playing(_) => Err(GameError::WrongState),
        }
    }

    pub fn start(&mut self, user: &User) -> Result<(), GameError> {
        if self
            .players
            .get(0)
            .map(|leader| *leader == *user)
            .unwrap_or_default()
        {
            match self.status {
                Status::Waiting => {
                    if self.players.len() >= 2 {
                        self.players.shuffle(&mut rand::thread_rng());
                        self.status = Status::Playing(Coloretto::new(self.players.len())?);
                        Ok(())
                    } else {
                        Err(GameError::NotEnoughPlayers)
                    }
                }
                Status::Playing(_) => Err(GameError::WrongState),
            }
        } else {
            Err(GameError::NotLeader)
        }
    }

    pub fn perform(&mut self, user: &User, action: Action) -> Result<(), GameError> {
        match &mut self.status {
            Status::Playing(coloretto) => {
                if self
                    .players
                    .iter()
                    .position(|u| *u == *user)
                    .map(|turn| turn == coloretto.turn())
                    .unwrap_or_default()
                {
                    coloretto.perform(action)?;
                    Ok(())
                } else {
                    Err(GameError::AwaitTurn)
                }
            }
            Status::Waiting => Err(GameError::WrongState),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Waiting,
    Playing(Coloretto),
}

#[derive(Debug, Error)]
pub enum GameError {
    #[error("that room does not exist")]
    Invalid,
    #[error("this room is full")]
    Full,
    #[error("you already joined this game")]
    AlreadyJoined,
    #[error("not enough players to start the game")]
    NotEnoughPlayers,
    #[error("only the room leader can start the game")]
    NotLeader,
    #[error("it's not your turn")]
    AwaitTurn,
    #[error("room is in the wrong state")]
    WrongState,
    #[error("something went wrong with coloretto")]
    Coloretto(#[from] coloretto::Error),
}
