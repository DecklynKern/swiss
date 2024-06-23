use crate::*;

fn try_create_pairing(s1: &mut Vec<PlayerID>, s2: &mut Vec<PlayerID>, already_played: &[Vec<PlayerID>]) -> Option<(PlayerID, PlayerID)> {

    let player1 = *s1.last().unwrap();
    let player2 = *s2.last().unwrap();

    // add other absolute criteria
    if !already_played[player1].contains(&player2) && !already_played[player2].contains(&player1) {

        s1.pop();
        s2.pop();

        Some((player1, player2))

    }
    else {
        None
    }
}

fn pair_bracket(move_down_players: PlayerIDList, resident_players: PlayerIDList, already_played: &[Vec<PlayerID>]) -> Option<(Vec<(PlayerID, PlayerID)>, PlayerIDList)> {
    
    let m0 = move_down_players.0.len();
    let residents_len = resident_players.0.len();

    if m0 == 0 && residents_len == 1 {
        return Some((Vec::new(), resident_players));
    }

    let max_pairs = ((resident_players.0.len() + m0) / 2).min(residents_len);
    let m1 = m0.min(max_pairs);

    let n1 = if m0 == 0 {max_pairs} else {m1};

    let mut s1 = Vec::new();
    let mut s2 = Vec::new();
    let mut limbo = Vec::new();

    for &id in &move_down_players.0[..(n1.min(m0))] {
        s1.push(id);
    }

    if n1 < m0 {
   
        for &id in &move_down_players.0[n1..] {
            limbo.push(id);
        }
    }

    let num_residents_in_s1 = n1.saturating_sub(m0);

    for &id in &resident_players.0[..num_residents_in_s1] {
        s1.push(id);
    }

    for &id in &resident_players.0[num_residents_in_s1..] {
        s2.push(id);
    }

    let mut accepted_pairings = Vec::new();

    let mut swap_idx = 1;

    while !s1.is_empty() {

        if let Some(pairing) = try_create_pairing(&mut s1, &mut s2, already_played) {
            accepted_pairings.push(pairing);
            swap_idx = 1;
        }
        else {

            if swap_idx == s2.len() {

                let next_move_downs: Vec<_> =
                    s2.iter()
                        .chain(limbo.iter())
                        .chain(s1.iter())
                        .cloned()
                        .collect();

                return Some((accepted_pairings, PlayerIDList(next_move_downs)));

            }

            let s2_len = s2.len() - 1;
            s2.swap(swap_idx, s2_len);
            swap_idx += 1;

        }
    }

    let next_move_downs: Vec<_> =
        s2.iter()
            .chain(limbo.iter())
            .cloned()
            .collect();

    Some((accepted_pairings, PlayerIDList(next_move_downs)))

}

fn pair_brackets(mut brackets: Vec<PlayerIDList>, move_down_players: PlayerIDList, already_played: &[Vec<PlayerID>]) -> Option<Vec<(PlayerID, PlayerID)>> {

    let Some(mut resident_players) = brackets.pop()
    else {

        let Some((pairs, new_move_down_players)) = pair_bracket(PlayerIDList(Vec::new()), move_down_players, already_played)
        else {
            return None;
        };

        if new_move_down_players.0.is_empty() {
            return Some(pairs);
        }
        else {
            return None;
        }
    };

    let pairs = loop {
        
        if let Some((mut new_pairs, new_move_down_players)) = pair_bracket(move_down_players.clone(), resident_players.clone(), already_played) {

            if let Some(mut recursive_pairs) = pair_brackets(brackets.clone(), new_move_down_players, already_played) {
                new_pairs.append(&mut recursive_pairs);
                break(new_pairs);
            }
        }

        let Some(mut new_list) = brackets.pop()
        else {
            return None;
        };

        resident_players.0.append(&mut new_list.0);

    };
    
    Some(pairs)

}

fn try_use_bye_player(bye_player: Option<PlayerID>, players_by_score: &PlayerIDList, scores: &[f32], tournament: &Tournament) -> Option<Vec<(PlayerID, PlayerID)>> {

    let mut using_players = players_by_score.clone();
    if let Some(bye) = bye_player {
        using_players.remove(bye);
    }

    let already_played = tournament.get_already_played();

    let mut brackets = Vec::new();
    let mut old_score = f32::MIN;
    
    for &player in using_players.0.iter() {
        
        if scores[player] != old_score {
            brackets.push(PlayerIDList(Vec::new()));
            old_score = scores[player];
        }

        brackets.last_mut().unwrap().0.push(player);

    }

    pair_brackets(brackets, PlayerIDList(Vec::new()), &already_played)
    
}

impl Round {

    pub fn generate_dutch(tournament: &Tournament) -> Self {

        let scores = tournament.get_player_scores();

        let mut players_by_score = tournament.get_active_player_ids();
        players_by_score.sort_by_scores_descending(&scores);
        
        let bye_players = players_by_score.get_players_without_bye(tournament);
        let mut bye_player = None;

        let pairs = if players_by_score.odd() {

            let mut result_pairs = None;

            for bye in bye_players {

                bye_player = Some(bye);
                result_pairs = try_use_bye_player(bye_player, &players_by_score, &scores, tournament);

                if result_pairs.is_some() {
                    break;
                }
            }

            result_pairs

        }
        else {
            try_use_bye_player(None, &players_by_score, &scores, tournament)
        };

        let Some(valid_pairs) = pairs
        else {
            println!("Warning: Was not able to find satisfiable pairings, using monrad to generate round");
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