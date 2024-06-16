use crate::*;

fn coin_flip() -> bool {
    rand::random::<bool>()
}

pub struct Pairing {
    pub white_player: PlayerID,
    pub black_player: PlayerID
}

impl Pairing {

    pub fn new(white_player: PlayerID, black_player: PlayerID) -> Self {
        Self {
            white_player,
            black_player
        }
    }

    pub fn randomized(player1: PlayerID, player2: PlayerID) -> Self {
        if coin_flip() {
            Self::new(player1, player2)
        }
        else {
            Self::new(player2, player1)
        }
    }

    pub fn using_colour_differences(player1: PlayerID, player2: PlayerID, colour_differences: &[i32]) -> Self {

        let colour_difference1 = colour_differences[player1];
        let colour_difference2 = colour_differences[player2];
    
        let mut grant_player1_preference = true;
    
        // add more conditions
        if colour_difference2.abs() > colour_difference1.abs() {
            grant_player1_preference = false;
        }
    
        let player1_is_white = if grant_player1_preference {
            colour_difference1 < 0
        }
        else {
            colour_difference2 > 0
        };
    
        if player1_is_white {
            Self::new(player1, player2)
        }
        else {
            Self::new(player2, player1)
        }
    }
}

pub fn make_pairings(pairs: &[(PlayerID, PlayerID)], pairing_func: impl Fn(PlayerID, PlayerID) -> Pairing) -> Vec<Pairing> {
    pairs.iter()
        .map(|(player1, player2)| pairing_func(*player1, *player2))
        .collect()
}