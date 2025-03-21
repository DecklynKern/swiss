mod player;
mod round;
mod pairing;
mod tournament;
mod algorithms;
mod error;

use player::*;
use round::*;
use tournament::*;
use pairing::*;
use error::*;

use std::io::{Read, Write};
use std::fs::File;

fn main() {

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut read_line = |prompt: &str, lower: bool| {

        print!("{}", prompt);
        let _ = stdout.flush();
        
        let mut line = String::new();
        let _ = stdin.read_line(&mut line);

        if (lower) {
            line = line.to_lowercase();
        }
        
        line.to_lowercase().trim().to_owned()

    };

    let args = std::env::args().collect::<Vec<_>>();

    let mut tournament = Tournament::new();

    if args.len() > 1 {

        println!("Reading player data from file: {}", args[1]);
        
        let Ok(mut file) = File::open(args[1].clone())
        else {
            error("File not found");
        };

        let mut lines = String::new();
        let _ = file.read_to_string(&mut lines);

        for line in lines.split('\n') {

            let mut parts = line.split(',');

            let name = parts.next().unwrap().trim().to_owned();

            let rating = parts.next().map(|string| {

                let Ok(rating_val) = string.trim().parse()
                else {
                    error(format!("Invalid elo \"{string}\""));
                };

                rating_val

            });

            tournament.add_player(name, rating);

        }
    };

    loop {

        let command = read_line("\n> ", true);
        let split: Vec<_> = command.split(' ').collect();

        match split[0] {
            "add" => {
                tournament.add_player(
                    read_line("Name: ", false),
                    read_line("Rating (leave blank for unknown): ", true).parse().ok()
                );
            }
            "remove" => {

                let name = if split.len() > 1 {
                    split[1..].join(" ")
                }
                else {
                    read_line("Name: ", false)
                };

                if !tournament.remove_player(&name) {
                    println!("Error: could not find player \"{name}\".");
                }
            }
            "standings" => {

                let mut player_ids = tournament.get_all_player_ids().0;

                let scores: Vec<_> =
                    player_ids.iter()
                        .cloned()
                        .map(|id| tournament.calc_score(id))
                        .collect();

                let sb_scores: Vec<_> =
                    player_ids.iter()
                        .map(|player| tournament.calc_sonneborn_berger_score(*player, &scores))
                        .collect();

                player_ids.sort_by(|&id1, &id2| {

                    if scores[id1] != scores[id2] {
                        scores[id2].total_cmp(&scores[id1])
                    }
                    else if sb_scores[id1] != sb_scores[id2] {
                        sb_scores[id2].total_cmp(&sb_scores[id1])
                    }
                    else {
                        0.cmp(&0) // fix
                    }
                });

                let mut stats: Vec<_> =
                    player_ids.iter()
                        .map(|_| (0, 0, 0, 0))
                        .collect();

                for round in tournament.rounds.iter() {

                    if let Some(bye_player) = round.bye_player {
                        stats[bye_player].3 += 1;
                    }
                    
                    for game in round.games.iter() {
                        match game.result {
                            GameResult::Win => {
                                stats[game.white_player].0 += 1;
                                stats[game.black_player].2 += 1;
                            }
                            GameResult::Draw => {
                                stats[game.white_player].1 += 1;
                                stats[game.black_player].1 += 1;
                            }
                            GameResult::Loss => {
                                stats[game.white_player].2 += 1;
                                stats[game.black_player].0 += 1;
                            }
                            _ => {}
                        }
                    }
                }

                let mut prev_score = f32::MIN;
                let mut prev_sb_score = f32::MIN;
                let mut placing = 0;

                println!("====Round {} Standings====", tournament.rounds.len());
                println!("## | Score | SB Score | W/D/L/B | Name");
                println!("---|-------|----------|---------|------------");

                for (idx, &id) in player_ids.iter().enumerate() {

                    let score = scores[id];
                    let sb_score = sb_scores[id];
                    let (wins, draws, losses, byes) = stats[id];

                    if score != prev_score || sb_score != prev_sb_score {
                        placing = idx + 1;
                        prev_score = score;
                        prev_sb_score = sb_score;
                    }

                    let withdraw_star = if tournament.players[id].active {
                        ' '
                    }
                    else {
                        '*'
                    };

                    println!("{: >2} | {score: >5.1}{withdraw_star}| {sb_score: >8.2} | {wins}/{draws}/{losses}/{byes} | {}", placing, tournament.players[id].name);
                }
            }
            "start" => {

                if let Some(prev_round) = tournament.rounds.last() {

                    let mut active_games = Vec::new();

                    for (idx, game) in prev_round.games.iter().enumerate() {
                        if game.result == GameResult::Pending {
                            active_games.push(idx);
                        }
                    }

                    if !active_games.is_empty() {

                        println!("Error: The following games are still ongoing, report a score for them:");
    
                        for game_idx in active_games {
                            prev_round.games[game_idx].print(&tournament.players);
                        }
    
                        continue;

                    }
                }

                let mut pairing_result = if !tournament.started() {
                    Round::from_seeding(&tournament)
                }
                else {
                    Round::generate_dutch(&tournament)
                };

                let scores = tournament.get_player_scores();
                pairing_result.games.sort_by(|game1, game2| {

                    let game1_score = scores[game1.white_player] + scores[game1.black_player];
                    let game2_score = scores[game2.white_player] + scores[game2.black_player];
                
                    game2_score.total_cmp(&game1_score)
                
                });

                for (idx, game) in pairing_result.games.iter_mut().enumerate() {
                    game.board_number = idx as u32 + 1;
                }

                println!("====Round {} Pairings====", tournament.rounds.len() + 1);
                println!("[Board #] White vs Black");
                println!("-------------------------------------");

                for pairing in pairing_result.games.iter() {
                    pairing.print(&tournament.players);
                }

                if let Some(bye_player) = pairing_result.bye_player {
                    println!("Bye: {}", tournament.players[bye_player].name)
                }

                tournament.rounds.push(pairing_result);

            }
            "reject" => {
                if read_line("Are you sure you want to reject the current round? ", true).to_lowercase().chars().nth(0).unwrap_or('n') == 'y' {
                    tournament.rounds.pop();
                }
            }
            "manual" => {
                
                let mut remaining_players = tournament.get_active_player_ids();
                let mut round_pairings: Vec<Pairing> = Vec::new();

                let mut add_round = true;

                while remaining_players.0.len() > 1 {

                    println!("\nCurrent pairings:");

                    for pairing in &round_pairings {
                        println!("{} vs. {}", tournament.players[pairing.white_player].name, tournament.players[pairing.black_player].name);
                    }

                    println!("Remaining players:");

                    for (idx, &player) in remaining_players.0.iter().enumerate() {
                        println!("({}) {}", idx, tournament.players[player].name);
                    }

                    let response = read_line("Enter white player number ('cancel' to cancel): ", true);

                    if response == "cancel" {
                        add_round = false;
                        break;
                    }

                    let Ok(player1_idx) = response.parse::<usize>()
                    else {
                        println!("Error: invalid index");
                        continue;
                    };

                    if player1_idx >= remaining_players.0.len() {
                        println!("Error: invalid index");
                        continue;
                    }

                    let Ok(player2_idx) = read_line("Enter black player number: ", true).parse::<usize>()
                    else {
                        println!("Error: invalid index");
                        continue;
                    };

                    if player2_idx >= remaining_players.0.len() {
                        println!("Error: invalid index");
                        continue;
                    }

                    if player1_idx == player2_idx {
                        println!("Error: indexes match");
                        continue;
                    }

                    round_pairings.push(Pairing::new(remaining_players.0[player1_idx], remaining_players.0[player2_idx]));

                    remaining_players.0.remove(player1_idx.max(player2_idx));
                    remaining_players.0.remove(player1_idx.min(player2_idx));

                }

                if add_round {
                    
                    let mut round = Round::from_pairings(round_pairings, remaining_players.0.pop());
                    
                    for board_num in 1..=round.games.len() {
                        round.games[board_num - 1].board_number = board_num as u32;
                    }

                    tournament.rounds.push(round);

                }
            }
            "report" => {

                let Some(round) = tournament.rounds.last_mut() else {
                    println!("Error: Tournament has not started.");
                    continue;
                };

                let number_text = if split.len() > 1 {
                    split[1].to_string()
                }
                else {
                    read_line("Board number: ", true)
                };

                let Ok(board_number) = number_text.parse::<u32>()
                else {
                    println!("Error: Invalid board number.");
                    continue;
                };

                let mut found_game = false;

                for game in round.games.iter_mut() {
                    if board_number == game.board_number {

                        let result_string = if split.len() > 2 {
                            split[2].to_string()
                        }
                        else {
                            read_line(&format!("Result for white player ({}) [W]in/[D]raw/[L]oss/[U]nreport: ", tournament.players[game.white_player].name), true)
                        };
                    
                        game.result = match result_string.to_lowercase().chars().next().unwrap() {
                            'w' => GameResult::Win,
                            'd' => GameResult::Draw,
                            'l' => GameResult::Loss,
                            'u' => GameResult::Pending,
                            _ => {
                                println!("Error: Invalid match result.");
                                break;
                            }
                        };
                    
                        found_game = true;
                        break;
                    
                    }
                }

                if !found_game {
                    println!("Error: No active game at board {board_number}.");
                }
            }
            "games" => {

                let Some(round) = tournament.rounds.last()
                else {
                    println!("Error: Tournament has not started.");
                    continue;    
                };

                for game in round.games.iter() {
                    game.print(&tournament.players);
                }
            }
            "export" => {

                let filename = if split.len() > 1 {
                    split[1..].join(" ")
                }
                else {
                    read_line("Filename: ", true)
                };

                let Ok(mut file) = File::options()
                    .write(true)
                    .create_new(true)
                    .open(filename)
                else {
                    println!("Error: File already exists");
                    continue;
                };
                
                for (idx, round) in tournament.rounds.iter().enumerate() {

                    let _ = file.write(format!("\nRound {}\n", idx + 1).as_bytes());

                    for game in round.games.iter() {
                        let _ = file.write(game.as_string(&tournament.players).as_bytes());
                        let _ = file.write("\n".as_bytes());
                    }
                }
                
            }
            "list" => {
                println!("Commands: [add, remove, standings, start, reject, manual, round, games, export, list]");
            }
            // testing only
            "sweep" => {

                let Some(round) = tournament.rounds.last_mut() else {
                    println!("Error: Tournament has not started.");
                    continue;
                };

                for game in round.games.iter_mut() {
                    game.result = GameResult::Win;
                }
            }
            _ => println!("Unknown command: {}", command)
        }
    }
}
