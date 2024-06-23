use crate::*;

fn top_pair_valid(pairs: &mut [(usize, usize)], already_played: &[Vec<PlayerID>]) -> bool {
    
    let Some(&(player1, player2)) = pairs.first()
    else {
        return true;
    };

    !has_already_played(player1, player2, already_played) && create_valid_pairs(&mut pairs[1..], already_played)
    
}

fn create_valid_pairs(pairs: &mut [(usize, usize)], already_played: &[Vec<PlayerID>]) -> bool {

    if pairs.is_empty() {
        return true;
    }

    if top_pair_valid(pairs, already_played) {
        return true;
    }

    if pairs.len() == 1 {
        return false;
    }

    for idx in 1..already_played.len() {

        let swap_a = pairs[0].0;
        let mut swap_b = pairs[idx].0;

        (pairs[0].0, pairs[idx].0) = (swap_b, swap_a);
        if top_pair_valid(pairs, already_played) {
            return true;
        }
        (pairs[0].0, pairs[idx].0) = (swap_a, swap_b);

        swap_b = pairs[idx].1;

        (pairs[0].0, pairs[idx].1) = (swap_b, swap_a);
        if top_pair_valid(pairs, already_played) {
            return true;
        }
        (pairs[0].0, pairs[idx].1) = (swap_a, swap_b);

    }

    false

}

fn try_use_bye_player(bye_player: Option<PlayerID>, players_by_score: &PlayerIDList, already_played: &[Vec<PlayerID>]) -> Option<Vec<(PlayerID, PlayerID)>> {

    let mut using_players = players_by_score.clone();
    if let Some(bye) = bye_player {
        using_players.remove(bye);
    }

    let mut pairs = using_players.pair_off_in_order();

    create_valid_pairs(&mut pairs, already_played).then_some(pairs)

}

impl Round {

    pub fn generate_monrad(tournament: &Tournament) -> Self {

        let scores = tournament.get_player_scores();
        
        let mut players_by_score = tournament.get_active_player_ids();
        players_by_score.sort_by_scores_descending(&scores);
        players_by_score.0.reverse();
        
        let bye_players = players_by_score.get_players_without_bye(tournament);
        let mut bye_player = None;

        let already_played = tournament.get_already_played();

        let pairs = if players_by_score.odd() {

            let mut result_pairs = None;

            for bye in bye_players {

                bye_player = Some(bye);
                result_pairs = try_use_bye_player(bye_player, &players_by_score, &already_played);

                if result_pairs.is_some() {
                    break;
                }
            }

            result_pairs

        }
        else {
            try_use_bye_player(None, &players_by_score, &already_played)
        };

        let Some(valid_pairs) = pairs
        else {
            println!("Warning: Was not able to find pairing without repeats, had to settle with having at least one.");
            return Self::generate_danish(tournament);
        };

        Self::from_pairings(
            make_pairings(
                &valid_pairs,
                tournament.colour_difference_pairing()
            ),
            bye_player
        )
    }
}