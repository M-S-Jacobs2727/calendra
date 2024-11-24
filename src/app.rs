mod card;
mod display;
mod field;
mod player;
mod season;
mod wins;

use rand::prelude::*;

use card::{AffectedCards, Card, Op, Rune, Score};
use field::Spot;
use player::{Deck, Player};
pub(crate) use season::Season;
use wins::*;

struct Play {
    pub player_turn: usize,
    pub played_field: usize,
    pub card_index: usize,
    pub spot: Spot,
}

pub struct App {
    players: Vec<Player>,
}
impl App {
    pub fn new() -> Self {
        Self { players: vec![] }
    }
    pub fn run(&mut self) {
        let num_players = display::get_num_players();
        assert_eq!(4, num_players, "Only 4 players are supported right now...");
        self.players = vec![
            Player::new(Season::Spring),
            Player::new(Season::Summer),
            Player::new(Season::Autumn),
            Player::new(Season::Winter),
        ];
        let decks = self.prepare_decks();
        self.players
            .iter_mut()
            .zip(decks.into_iter())
            .for_each(|(p, d)| p.set_deck(d));
        let win = self.game_loop();
        display::game_over(self.players[win.player_idx].season(), win.condition);
    }
    fn game_loop(&mut self) -> Win {
        let mut first_player = 0usize;
        loop {
            let win = self.play_round(first_player);
            if win.game_won {
                return win;
            }
            first_player = win.player_idx;
            self.complete_round(win);
        }
    }
    fn prepare_decks(&self) -> [Deck; 4] {
        let mut all_cards = Card::load_all("./assets/card_list.csv");
        assert_eq!(120, all_cards.len());
        all_cards.shuffle(&mut rand::thread_rng());
        let p1 = all_cards.split_off(90);
        let p2 = all_cards.split_off(60);
        let p3 = all_cards.split_off(30);
        [p1, p2, p3, all_cards]
    }
    fn complete_round(&mut self, win: Win) {
        let winner = win.player_idx;
        let prizes: Vec<&Card> = self
            .players
            .iter()
            .map(|p| p.prize().as_ref().expect("No prize?"))
            .collect();
        let seasons: Vec<Season> = self.players.iter().map(|p| p.season()).collect();
        let choice = display::choose_prize(winner, prizes, seasons);
        if winner != choice {
            let mid = winner.max(choice);
            let min = winner.min(choice);
            let (a, b) = self.players.split_at_mut(mid);
            a[min].swap_prizes(&mut b[0]);
        }

        for i in 0..self.players.len() {
            let player = &mut self.players[i];
            player.move_hand_to_deck();
            player.reset_field();
            let prize = player.take_prize();
            if i == winner && winner != choice && prize.season() == player.season() {
                player.add_to_hand(prize);
            } else {
                player.add_to_deck(prize);
            }
            player.shuffle_deck();
        }

        display::round_over(self.players[winner].season(), win.condition);
    }
    fn play_round(&mut self, first_player: usize) -> Win {
        let num_players = self.players.len();
        assert!(first_player < num_players);
        self.init_round();
        let mut player_turn = first_player;
        loop {
            let play = display::select_play(player_turn, &self.players);
            self.execute_play(&play);

            if let Some(win) = self.check_win(&play) {
                return win;
            }
            player_turn = (player_turn + 1) % num_players;
        }
    }
    fn init_round(&mut self) {
        for player in &mut self.players {
            player.fill_hand();
            player.show_prize();
        }
    }

    fn execute_play(&mut self, play: &Play) {
        let card = self.players[play.player_turn].take_from_hand(play.card_index);
        let other_card = self.players[play.played_field].play_card(card, play.spot);
        if let Some(c) = other_card {
            self.players[play.player_turn].add_to_hand(c);
        }
    }
    fn check_win(&self, play: &Play) -> Option<Win> {
        let player = &self.players[play.played_field];
        let player_idx = play.played_field;
        let spot = play.spot;
        let card = player
            .field()
            .get(play.spot)
            .as_ref()
            .expect("Should be a card here from excuting play");

        if card.season() == player.season() {
            let field_in_season = player.field().clone_in_season(player.season());

            // If there is a win condition on the in-season field, then it is a game win
            let opt_win_cond = check_win(&field_in_season, &spot, card);
            if let Some(condition) = opt_win_cond {
                return Some(Win {
                    player_idx,
                    game_won: true,
                    condition,
                });
            }
        }

        let opt_win_cond = check_win(player.field(), &spot, card);
        if let Some(condition) = opt_win_cond {
            let game_won =
                check_two_ancients_house_rule(&player.field().court, &condition, player.season());
            Some(Win {
                player_idx,
                game_won,
                condition,
            })
        } else {
            None
        }
    }
}
