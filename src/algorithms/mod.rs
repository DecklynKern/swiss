mod monrad;
mod dutch;
mod danish;

use crate::*;

impl Round {

    pub fn from_pairings(pairings: Vec<Pairing>, bye_player: Option<PlayerID>) -> Self {

        let round = Self {
            games: pairings.iter()
                .enumerate()
                .map(|(idx, pairing)| Game {
                    white_player: pairing.white_player,
                    black_player: pairing.black_player,
                    result: GameResult::Pending,
                    board_number: idx + 1
                })
                .collect::<Vec<_>>(),
            bye_player
        };

        round

    }

    pub fn from_seeding(tournament: &Tournament) -> Self {

        let mut players_by_seeding = tournament.get_active_player_ids();
        players_by_seeding.0.sort_by(
            |&id1, &id2| tournament.players[id2].rating.cmp(&tournament.players[id1].rating)
        );

        let bye_player = (players_by_seeding.odd()).then(|| players_by_seeding.0.pop().unwrap());

        Self::from_pairings(
            make_pairings(
                &players_by_seeding.pair_off_alternating_sides(),
                Pairing::randomized
            ),
            bye_player
        )
    }
}