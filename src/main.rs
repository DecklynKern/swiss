mod player;
mod round;
mod pairing;
mod util;

use player::*;
use round::*;
use pairing::*;
use util::*;

use std::io::{Read, Write};

fn main() {

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut read_line = |prompt: &str| {

        print!("{}", prompt);
        let _ = stdout.flush();
        
        let mut line = String::new();
        let _ = stdin.read_line(&mut line);
        
        line.to_lowercase().trim().to_owned()

    };

    let args = std::env::args().collect::<Vec<_>>();

    let mut players: Vec<Player> = Vec::new();

    if args.len() >= 1 {

        println!("Reading player data from file: {}", args[1]);
        
        let Ok(mut file) = std::fs::File::open(args[1].clone())
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

            players.push(Player::new(name, rating));

        }
    };
    
    let mut rounds: Vec<Round> = Vec::new();

    loop {

        let command = read_line("> ");

        match command.as_str() {
            "add player" => {

                players.push(Player::new(
                    read_line("Name: "),
                    read_line("Rating (leave blank for unknown): ").parse().ok()
                ));
            }
            "remove player" => {

                let name = read_line("Name: ");
                let mut found = false;

                for player in players.iter_mut() {
                    if player.name == name {
                        player.active = false;
                        found = true;
                        break;
                    }
                }

                if !found {
                    println!("Error: could not find player \"{name}\".");
                }
            }
            "standings" => {

                let mut player_ids: Vec<_> = (0..players.len()).collect();

                let scores: Vec<_> =
                    player_ids.iter()
                        .cloned()
                        .map(|id| calc_score(id, &rounds))
                        .collect();

                let sb_scores: Vec<_> =
                    player_ids.iter()
                        .map(|player| calc_sonneborn_berger_score(*player, &scores, &rounds))
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

                for round in rounds.iter() {

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

                println!("====Round {} Standings====", rounds.len());
                println!("## | Score | SB Score | W/D/L/B | Name");
                println!("---|-------|----------|---------|------------");

                for (idx, &id) in player_ids.iter().enumerate() {

                    let score = scores[id];
                    let sb_score = sb_scores[id];
                    let (wins, draws, losses, byes) = stats[id];

                    println!("{: >2} | {score: >5.1} | {sb_score: >8.2} | {wins}/{draws}/{losses}/{byes} | {}", idx + 1, players[id].name);
                }
            }
            "start round" => {

                if let Some(prev_round) = rounds.last() {

                    let mut active_games = Vec::new();

                    for (idx, game) in prev_round.games.iter().enumerate() {
                        if game.result == GameResult::Pending {
                            active_games.push(idx);
                        }
                    }

                    if !active_games.is_empty() {

                        println!("Error: The following games are still ongoing, report a score for them:");
    
                        for game_idx in active_games {
                            prev_round.games[game_idx].print(&players);
                        }
    
                        continue;

                    }
                }

                let pairing_result = if rounds.is_empty() {
                    generate_round_1(&players)
                }
                else {
                    generate_round_dutch(&players, &rounds)
                };

                println!("====Round {} Pairings====", rounds.len() + 1);
                println!("(Board #) White vs Black");
                println!("-------------------------------------");

                for pairing in pairing_result.games.iter() {
                    pairing.print(&players);
                }

                if let Some(bye_player) = pairing_result.bye_player {
                    println!("Bye: {}", players[bye_player].name)
                }

                rounds.push(pairing_result);

            }
            "report" => {

                let Some(round) = rounds.last_mut() else {
                    println!("Error: Tournament has not started");
                    continue;
                };

                let Ok(board_number) = read_line("Enter board number: ").parse::<usize>()
                else {
                    println!("Error: Invalid board number.");
                    continue;
                };

                let mut found_game = false;

                for game in round.games.iter_mut() {
                    if board_number == game.board_number {

                        let result_string = read_line(&format!("Enter result for white player ({}) [W]in/[D]raw/[L]oss/[U]nreport: ", players[game.white_player].name));
                    
                        game.result = match result_string.to_lowercase().chars().nth(0).unwrap() {
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
            _ => println!("Unknown command: {command}")
        }
    }
}
