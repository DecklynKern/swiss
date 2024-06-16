// use crate::player::*;
// use crate::round::*;
// use crate::error::*;

// fn try_create_pairing(s1: &mut Vec<PlayerID>, s2: &mut Vec<PlayerID>, already_played: &[Vec<PlayerID>]) -> Option<(PlayerID, PlayerID)> {

//     let player1 = *s1.last().unwrap();
//     let player2 = *s2.last().unwrap();

//     // add other absolute criteria
//     if !already_played[player1].contains(&player2) && !already_played[player2].contains(&player1) {

//         s1.pop();
//         s2.pop();

//         Some((player1, player2))

//     }
//     else {
//         None
//     }
// }

// fn pair_bracket(move_down_players: Vec<PlayerID>, resident_players: Vec<PlayerID>, players: &[Player], already_played: &[Vec<PlayerID>]) -> (Vec<(PlayerID, PlayerID)>, Vec<PlayerID>) {

//     if move_down_players.is_empty() && resident_players.len() == 1 {
//         return (Vec::new(), resident_players);
//     }

//     let m0 = move_down_players.len();
//     let max_pairs = ((resident_players.len() + m0) / 2).min(resident_players.len());
//     let m1 = m0.min(max_pairs);

//     let n1 = if m0 == 0 {max_pairs} else {m1};

//     let mut s1 = Vec::new();
//     let mut s2 = Vec::new();
//     let mut limbo = Vec::new();

//     for &id in &move_down_players[..(n1.min(move_down_players.len()))] {
//         s1.push(id);
//     }

//     if n1 < move_down_players.len() {
   
//         for &id in &move_down_players[n1..] {
//             limbo.push(id);
//         }
//     }

//     let num_residents_in_s1 = n1 - m0;

//     for &id in &resident_players[..num_residents_in_s1] {
//         s1.push(id);
//     }

//     for &id in &resident_players[num_residents_in_s1..] {
//         s2.push(id);
//     }

//     let mut accepted_pairings = Vec::new();

//     let mut swap_idx = 1;

//     while !s1.is_empty() {

//         if let Some(pairing) = try_create_pairing(&mut s1, &mut s2, already_played) {
//             accepted_pairings.push(pairing);
//             swap_idx = 1;
//         }
//         else {

//             if swap_idx == s2.len() {
//                 error("Could not find a pairing, crying about it.");
//             }

//             s2.swap(0, swap_idx);
//             swap_idx += 1;

//         }
//     }

//     let next_move_downs: Vec<_> =
//         s2.iter()
//             .chain(limbo.iter())
//             .cloned()
//             .collect();

//     (accepted_pairings, next_move_downs)

// }

// pub fn generate_round_dutch(players: &[Player], prev_rounds: &[Round]) -> Round {

//     let mut player_ids: Vec<_> =
//         (0..players.len())
//             .filter(|&id| players[id].active)
//             .collect();

//     let scores: Vec<_> =
//         (0..players.len())
//             .map(|id| calc_score(id, prev_rounds))
//             .collect();

//     //sort_by_scores(&mut player_ids, &scores);

//     let bye_players: Vec<_> =
//         prev_rounds.iter()
//             .filter_map(|round| round.bye_player)
//             .collect();

//     let mut already_played = vec![Vec::new(); players.len()];
//     let mut colour_differences = vec![0; players.len()];

//     for round in prev_rounds.iter() {
//         for game in round.games.iter() {

//             already_played[game.white_player].push(game.black_player);
//             already_played[game.black_player].push(game.white_player);

//             colour_differences[game.white_player] += 1;
//             colour_differences[game.black_player] -= 1;

//         }
//     }

//     let mut pairing_brackets = Vec::new();
//     let mut current_score = f32::MAX;

//     for id in player_ids {

//         if scores[id] != current_score {
//             pairing_brackets.push(Vec::new());
//         }

//         pairing_brackets.last_mut().unwrap().push(id);
//         current_score = scores[id];

//     }

//     let mut move_down_players = Vec::new();
//     let mut final_pairings = Vec::new();

//     while let Some(pairing_bracket) = pairing_brackets.pop() {

//         let (new_pairings, unpaired_players) = pair_bracket(move_down_players, pairing_bracket, players, &already_played);

//         move_down_players = unpaired_players;

//         for pairing in new_pairings {
//             final_pairings.push(pairing);
//         }
//     }

//     if move_down_players.len() > 1 {
//         error("massive bruh moment");
//     }

//     let games: Vec<_> =
//         final_pairings.iter()
//         .enumerate()
//         .map(|(idx, &(id2, id1))|
//             create_pairing_with_optimal_colours(id1, colour_differences[id1], id2, colour_differences[id2], idx + 1))
//         .collect();

//     Round {
//         games,
//         bye_player: move_down_players.pop()
//     }
// }