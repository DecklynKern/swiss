use crate::player::*;
use crate::round::*;
use crate::pairing::*;

pub struct Tournament {
    pub players: Vec<Player>,
    pub rounds: Vec<Round>
}

impl Tournament {

    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            rounds: Vec::new()
        }
    }

    pub fn started(&self) -> bool {
        !self.rounds.is_empty()
    }

    pub fn add_player(&mut self, name: String, rating: Option<u32>) {
        self.players.push(Player::new(name, rating));
    }

    pub fn remove_player(&mut self, name: &str) -> bool {
        
        let mut found = false;

        for player in self.players.iter_mut() {
            if player.name.to_lowercase() == *name {
                player.active = false;
                found = true;
                break;
            }
        }

        found

    }

    pub fn get_all_player_ids(&self) -> PlayerIDList {
        PlayerIDList(
            (0..self.players.len()).collect()
        )
    }

    pub fn get_active_player_ids(&self) -> PlayerIDList {
        PlayerIDList(
            (0..self.players.len())
                .filter(|&id| self.players[id].active)
                .collect()
        )
    }

    pub fn get_bye_players(&self) -> Vec<PlayerID> {
        self.rounds.iter()
            .filter_map(|round| round.bye_player)
            .collect()
    }

    pub fn colour_difference_pairing(&self) -> impl Fn(PlayerID, PlayerID) -> Pairing + '_ {
    
        let mut colour_differences = vec![0; self.players.len()];
    
        for round in self.rounds.iter() {
            for game in round.games.iter() {
    
                colour_differences[game.white_player] += 1;
                colour_differences[game.black_player] -= 1;
    
            }
        }
    
        move |player1, player2| Pairing::using_colour_differences(player1, player2, &colour_differences)
    
    }

    pub fn get_already_played(&self) -> Vec<Vec<PlayerID>> {
    
        let mut already_played = vec![Vec::new(); self.players.len()];
    
        for round in self.rounds.iter() {
            for game in round.games.iter() {
    
                already_played[game.white_player].push(game.black_player);
                already_played[game.black_player].push(game.white_player);
    
            }
        }
    
        already_played
    
    }

    pub fn calc_score(&self, player: PlayerID) -> f32 {
    
        self.rounds.iter()
            .map(|round| {
    
                if let Some(bye_player) = round.bye_player {
                    if bye_player == player {
                        return GameResult::Win.score();
                    }
                }
            
                for game in round.games.iter() {
                    if game.white_player == player {
                        return game.result.score();
                    }
                    else if game.black_player == player {
                        return game.result.opposite().score();
                    }
                }
    
                0.0
    
            })
            .sum()
    }
    
    pub fn calc_sonneborn_berger_score(&self, player: PlayerID, player_scores: &[f32]) -> f32 {
    
        self.rounds.iter()
            .map(|round| {
    
                for game in round.games.iter() {
                    if game.white_player == player {
                        return game.result.score() * player_scores[game.black_player];
                    }
                    else if game.black_player == player {
                        return game.result.opposite().score() * player_scores[game.white_player];
                    }
                }
    
                0.0
    
            })
            .sum()
    }
    
    pub fn get_player_scores(&self) -> Vec<f32> {
        (0..self.players.len())
            .map(|id| self.calc_score(id))
            .collect()
    }
}