use crate::player::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Win,
    Draw,
    Loss,
    Pending
}

impl GameResult {

    pub fn score(self) -> f32 {
        match self {
            Self::Win => 1.0,
            Self::Draw => 0.5,
            _ => 0.0
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Self::Win => Self::Loss,
            Self::Draw => Self::Draw,
            Self::Loss => Self::Win,
            Self::Pending => Self::Pending
        }
    }

    pub fn as_letter(self) -> char {
        match self {
            Self::Win => 'W',
            Self::Draw => 'D',
            Self::Loss => 'L',
            Self::Pending => ' '
        }
    }
}

pub struct Game {
    pub white_player: PlayerID,
    pub black_player: PlayerID,
    pub board_number: PlayerID,
    pub result: GameResult
}

impl Game {

    pub fn print(&self, players: &[Player]) {
    
        if self.result == GameResult::Pending {
            println!("[{}] {} vs. {}", self.board_number, players[self.white_player].name, players[self.black_player].name);
        }
        else {
            println!("[{}] {} ({}) vs {} ({})", self.board_number, players[self.white_player].name, self.result.as_letter(), players[self.black_player].name, self.result.opposite().as_letter());
        }
    }
}

pub struct Round {
    pub games: Vec<Game>,
    pub bye_player: Option<PlayerID>
}

pub fn has_already_played(player1: PlayerID, player2: PlayerID, already_played: &[PlayerIDList]) -> bool {
    already_played[player1].0.contains(&player2) || already_played[player2].0.contains(&player1)
}