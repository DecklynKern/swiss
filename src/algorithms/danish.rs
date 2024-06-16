use crate::*;

impl Round {

    pub fn generate_danish(tournament: &Tournament) -> Self {

        let scores = tournament.get_player_scores();

        let mut players_by_score = tournament.get_active_player_ids();
        players_by_score.sort_by_scores_ascending(&scores);
        players_by_score.0.reverse();
        
        let bye_player = players_by_score.odd().then(|| {
            let player = players_by_score.get_first_player_without_bye(tournament);
            players_by_score.remove(player);
            player
        });

        Self::from_pairings(
            make_pairings(
                &players_by_score.pair_off_in_order(),
                tournament.colour_difference_pairing()
            ),
            bye_player
        )
    }
}