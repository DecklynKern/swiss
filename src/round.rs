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

    pub fn as_string(&self, players: &[Player]) -> String {
    
        if self.result == GameResult::Pending {
            format!("[{}] {} vs. {}", self.board_number, players[self.white_player].name, players[self.black_player].name)
        }
        else {
            format!("[{}] {} ({}) vs {} ({})", self.board_number, players[self.white_player].name, self.result.as_letter(), players[self.black_player].name, self.result.opposite().as_letter())
        }
    } 

    pub fn print(&self, players: &[Player]) {
        println!("{}", self.as_string(players));
    }
}

pub struct Round {
    pub games: Vec<Game>,
    pub bye_player: Option<PlayerID>
}

pub fn has_already_played(player1: PlayerID, player2: PlayerID, already_played: &[Vec<PlayerID>]) -> bool {
    already_played[player1].contains(&player2) || already_played[player2].contains(&player1)
}