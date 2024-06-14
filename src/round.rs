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
    pub white_player: usize,
    pub black_player: usize,
    pub board_number: usize,
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
    pub bye_player: Option<usize>
}

pub fn calc_score(player_id: usize, rounds: &[Round]) -> f32 {

    rounds.iter()
        .map(|round| {

            if let Some(bye_player) = round.bye_player {
                if bye_player == player_id {
                    return GameResult::Win.score();
                }
            }
        
            for game in round.games.iter() {
                if game.white_player == player_id {
                    return game.result.score();
                }
                else if game.black_player == player_id {
                    return game.result.opposite().score();
                }
            }

            0.0

        })
        .sum()
}

pub fn calc_sonneborn_berger_score(player_id: usize, player_scores: &[f32], rounds: &[Round]) -> f32 {

    rounds.iter()
        .map(|round| {

            for game in round.games.iter() {
                if game.white_player == player_id {
                    return game.result.score() * player_scores[game.black_player];
                }
                else if game.black_player == player_id {
                    return game.result.opposite().score() * player_scores[game.white_player];
                }
            }

            0.0

        })
        .sum()
}