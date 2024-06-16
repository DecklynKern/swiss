use crate::tournament::*;

pub type PlayerID = usize;

pub struct Player {
    pub name: String,
    pub rating: Option<u32>,
    pub active: bool
}

impl Player {

    pub fn new(name: String, rating: Option<u32>) -> Self {
        
        Self {
            name,
            rating,
            active: true
        }
    }
}

#[derive(Clone)]
pub struct PlayerIDList(pub Vec<PlayerID>);

impl PlayerIDList {

    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn odd(&self) -> bool {
        self.0.len() % 2 == 1
    }

    pub fn remove(&mut self, player: PlayerID) {
        self.0.retain(|id| *id != player);
    }

    pub fn get_players_without_bye(&self, tournament: &Tournament) -> Vec<PlayerID> {
        
        let bye_players = tournament.get_bye_players();

        (0..tournament.players.len())
            .filter(|id| !bye_players.contains(id))
            .collect()

    }

    // assumes no player can have 2 byes
    pub fn get_first_player_without_bye(&self, tournament: &Tournament) -> PlayerID {
        let bye_players = tournament.get_bye_players();
        self.get_first_player_not_in_list(&bye_players).unwrap()
    }

    pub fn get_first_player_not_in_list(&self, exclude: &[PlayerID]) -> Option<PlayerID> {
        
        for player in self.0.iter() {
            if !exclude.contains(player) {
                return Some(*player);
            }
        }

        None

    }

    pub fn sort_by_scores_descending(&mut self, scores: &[f32]) {
        self.0.sort_by(|&id1, &id2| scores[id1].total_cmp(&scores[id2]));
    }

    pub fn sort_by_scores_ascending(&mut self, scores: &[f32]) {
        self.0.sort_by(|&id1, &id2| scores[id2].total_cmp(&scores[id1]));
    }

    pub fn pair_off_in_order(&self) -> Vec<(PlayerID, PlayerID)> {
        self.0.chunks(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect()
    }

    pub fn pair_off_alternating_sides(&self) -> Vec<(PlayerID, PlayerID)> {

        let mid_point = self.0.len() / 2;
        
        self.0[..mid_point]
            .iter()
            .cloned()
            .zip(self.0[mid_point..]
                .iter()
                .rev()
                .cloned())
            .collect()
    
    }
}