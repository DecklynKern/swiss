use crate::*;
use itertools::Itertools;

impl Round {

    pub fn generate_dutch(tournament: &Tournament) -> Self {

        let scores = tournament.get_player_scores();

        let mut players_by_score = tournament.get_active_player_ids();
        players_by_score.sort_by_scores_ascending(&scores);
        let mut bye_player = None;

        let pairs = if players_by_score.odd() {
        
            let mut bye_players = players_by_score.get_players_without_bye(tournament);
            bye_players.reverse();

            let mut result_pairs = None;

            for bye in bye_players {

                bye_player = Some(bye);
                result_pairs = try_use_bye_player(bye_player, &players_by_score, tournament);

                if result_pairs.is_some() {
                    break;
                }
            }

            result_pairs

        }
        else {
            try_use_bye_player(None, &players_by_score, tournament)
        };

        let Some(valid_pairs) = pairs
        else {
            println!("Warning: Was not able to find good pairing, settling with monrad pairings (potential repeats/bad matches).");
            return Self::generate_monrad(tournament);
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

fn try_use_bye_player(bye_player: Option<PlayerID>, players_by_score: &PlayerIDList, tournament: &Tournament) -> Option<Vec<(PlayerID, PlayerID)>> {

    let scores = tournament.get_player_scores();

    let mut using_players = players_by_score.clone();
    if let Some(bye) = bye_player {
        using_players.remove(bye);
    }

    let already_played = tournament.get_already_played();
    
    let mut pairing_brackets = Vec::new();
    let mut current_score = f32::MIN;

    for &id in &using_players.0 {

        if scores[id] != current_score {
            pairing_brackets.push(PlayerIDList::new());
        }

        pairing_brackets.last_mut().unwrap().0.push(id);
        current_score = scores[id];

    }

    // println!("BYE");
    // println!("{bye_player:?}");

    // println!("PLAYERS");
    // println!("{using_players:?}");

    // println!("SCORES");
    // println!("{scores:?}");
    // println!("BRACKETS");
    // println!("{pairing_brackets:?}");

    try_pair_brackets(pairing_brackets, &already_played)

}

fn try_pair_brackets(mut brackets: Vec<PlayerIDList>, already_played: &[PlayerIDList]) -> Option<Vec<(PlayerID, PlayerID)>> {

    let mut move_down_players = PlayerIDList::new();
    let mut final_pairings = Vec::new();

    for bracket in brackets.iter() {

        let (new_pairings, unpaired_players) = pair_bracket(move_down_players, bracket.clone(), &already_played);

        move_down_players = unpaired_players;

        for pairing in new_pairings {
            final_pairings.push(pairing);
        }
    }

    // println!("FINAL PAIRINGS");
    // println!("{final_pairings:?}");

    if move_down_players.0.is_empty() {
        Some(final_pairings)
    }
    else if brackets.len() > 1 {
        
        let bracket1 = brackets.pop().unwrap();
        let bracket2 = brackets.pop().unwrap();

        brackets.push(bracket1 + bracket2);
        try_pair_brackets(brackets, already_played)
    
    }
    else {
        None
    }
}

fn pair_bracket(move_down_players: PlayerIDList, resident_players: PlayerIDList, already_played: &[PlayerIDList]) -> (Vec<(PlayerID, PlayerID)>, PlayerIDList) {

    if move_down_players.0.is_empty() && resident_players.0.len() == 1 {
        return (Vec::new(), resident_players);
    }

    let m0 = move_down_players.0.len();
    let max_pairs = ((resident_players.0.len() + m0) / 2).min(resident_players.0.len());
    let m1 = m0.min(max_pairs);

    let n1 = if m0 == 0 {max_pairs} else {m1};

    let mut accepted_pairings = Vec::new();
    let mut next_move_downs = PlayerIDList::new();

    'outer: for move_down_players_permutation in move_down_players.0.iter().cloned().permutations(m0) {

        for resident_players_permutation in resident_players.0.iter().cloned().permutations(resident_players.0.len()) {

            let mut s1 = PlayerIDList::new();
            let mut s2 = PlayerIDList::new();
            let mut limbo = PlayerIDList::new();
        
            for &id in &move_down_players_permutation[..n1.min(m0)] {
                s1.0.push(id);
            }
        
            if n1 < m0 {
        
                for &id in &move_down_players_permutation[n1..] {
                    limbo.0.push(id);
                }
            }
        
            let num_residents_in_s1 = max_pairs.saturating_sub(m0);
        
            for &id in &resident_players_permutation[..num_residents_in_s1] {
                s1.0.push(id);
            }
        
            for &id in &resident_players_permutation[num_residents_in_s1..] {
                s2.0.push(id);
            }
        
            accepted_pairings = Vec::new();
            let mut successful = true;
        
            while !s1.0.is_empty() {
        
                if let Some(pairing) = try_create_pairing(&mut s1, &mut s2, already_played) {
                    accepted_pairings.push(pairing);
                }
                else {
                    successful = false;
                    break;
                }
            }
        
            next_move_downs = PlayerIDList(
                s1.0.iter()
                    .chain(s2.0.iter())
                    .chain(limbo.0.iter())
                    .cloned()
                    .collect());

            if successful {
                break 'outer;
            }
        
            // println!("\n{s1:?}");
            // println!("{s2:?}");
        
            // println!("{move_down_players:?}");
            // println!("{resident_players:?}");
            // println!("{already_played:?}");
            // println!("m0: {m0}, max_pairs: {max_pairs}, m1: {m1}, n1: {n1}");
            // println!("{accepted_pairings:?}");
            // println!("{next_move_downs:?}\n");

        }
    }
    
    (accepted_pairings, next_move_downs)

}

fn try_create_pairing(s1: &mut PlayerIDList, s2: &mut PlayerIDList, already_played: &[PlayerIDList]) -> Option<(PlayerID, PlayerID)> {

    let player1 = *s1.0.last().unwrap();
    let player2 = *s2.0.last().unwrap();

    // add other absolute criteria
    if !already_played[player1].0.contains(&player2) && !already_played[player2].0.contains(&player1) {

        s1.0.pop();
        s2.0.pop();

        Some((player1, player2))

    }
    else {
        None
    }
}
