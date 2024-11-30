mod card;
mod display;
mod field;
mod player;
mod season;
mod turn;
mod win_condition;

use card::{all_cards, Card};
use field::{Field, Spot};
use player::Player;
use season::Season;
use turn::Turn;
use win_condition::{check_two_ancients_house_rule, check_win, WinCondition};

use rand::prelude::*;

#[derive(PartialEq, Debug)]
struct WinState {
    pub player_index: usize,
    pub game_won: bool,
    pub condition: WinCondition,
}

pub struct App {
    players: Vec<Player>,
}
impl App {
    /// Create the game object
    pub fn new() -> Self {
        Self { players: vec![] }
    }
    /// Use this method to run the game
    pub fn run(&mut self) {
        let num_players = display::get_num_players();

        // TODO: support 2 or 3 players in a game
        assert_eq!(4, num_players, "Only 4 players are supported right now...");
        self.players = vec![
            Player::new(Season::Spring),
            Player::new(Season::Summer),
            Player::new(Season::Autumn),
            Player::new(Season::Winter),
        ];

        // Shuffles and distributes the decks to the players
        self.prepare_decks();

        // Once the game ends, use the win state to display a message
        let win_state = self.game_loop();
        display::game_over(
            self.players[win_state.player_index].season(),
            win_state.condition,
        );
    }
    fn num_players(&self) -> usize {
        self.players.len()
    }
    /// The main game loop that runs over multiple rounds
    fn game_loop(&mut self) -> WinState {
        let mut first_player = 0usize;
        loop {
            let win_state = self.play_round(first_player);
            if win_state.game_won {
                return win_state;
            }
            first_player = win_state.player_index;
            self.complete_round(win_state);
        }
    }
    /// Load all 120 cards, shuffle them together, and distribute decks to the players
    fn prepare_decks(&mut self) {
        let mut all_cards = all_cards();
        assert_eq!(120, all_cards.len());
        all_cards.shuffle(&mut rand::thread_rng());

        let player1_deck = all_cards.split_off(90);
        let player2_deck = all_cards.split_off(60);
        let player3_deck = all_cards.split_off(30);
        let player4_deck = all_cards;

        let decks = [player1_deck, player2_deck, player3_deck, player4_deck];
        self.players
            .iter_mut()
            .zip(decks.into_iter())
            .for_each(|(p, d)| p.set_deck(d));
    }
    /// After a round is over, the winner chooses a prize, cards in the hands
    /// and fields are shuffled back into the decks, and a new round will begin
    fn complete_round(&mut self, win_state: WinState) {
        let winning_player_index = win_state.player_index;
        display::round_over(
            self.players[winning_player_index].season(),
            win_state.condition,
        );

        let prizes: Vec<&Card> = self
            .players
            .iter()
            .map(|p| p.prize().as_ref().expect("No prize?"))
            .collect();
        let seasons: Vec<Season> = self.players.iter().map(|p| p.season()).collect();
        let chosen_prize_index = display::choose_prize(winning_player_index, prizes, seasons);
        if winning_player_index != chosen_prize_index {
            let prize1 = self.players[winning_player_index].take_prize();
            let prize2 = self.players[chosen_prize_index].take_prize();
            self.players[winning_player_index].set_prize(prize2);
            self.players[chosen_prize_index].set_prize(prize1);
        }

        for i in 0..self.players.len() {
            let player = &mut self.players[i];

            player.move_hand_to_deck();
            player.remove_cards_from_field();

            let prize = player.take_prize();

            // If the winning player swaps prizes and the received prize is
            // of their season, it is added to their hand instead of their deck
            if i == winning_player_index
                && winning_player_index != chosen_prize_index
                && prize.season() == player.season()
            {
                player.add_card_to_hand(prize);
            } else {
                player.add_card_to_deck(prize);
            }

            player.shuffle_deck();
        }
    }
    /// Players take turns selecting a card to play and a location in which
    /// to play it, and a win condition is checked based on the card that was played
    /// for the player whose field the card was played in.
    fn play_round(&mut self, first_player: usize) -> WinState {
        assert!(first_player < self.num_players());
        let mut player_index = first_player;

        self.initialize_round();
        loop {
            let turn = self.player_takes_turn(player_index);
            self.execute_turn(&turn);

            if let Some(win_state) = self.check_for_win_conditions(&turn) {
                return win_state;
            }
            player_index = (player_index + 1) % self.num_players();
        }
    }
    fn player_takes_turn(&self, player_index: usize) -> Turn {
        display::wait_for_next_player(self.players[player_index].season());

        let players_starting_with_self: Vec<&Player> = self
            .players
            .iter()
            .cycle()
            .skip(player_index)
            .take(self.num_players())
            .collect();
        let fields: Vec<&Field> = players_starting_with_self
            .iter()
            .map(|p| p.field())
            .collect();
        let seasons: Vec<Season> = players_starting_with_self
            .iter()
            .map(|p| p.season())
            .collect();
        let hand = players_starting_with_self[0].hand();
        display::show_all_fields(&fields);
        loop {
            let card_index_in_hand: usize = display::get_card_choice_from_hand(hand);
            let selected_card = &hand[card_index_in_hand];
            let valid_spots = turn::get_valid_spots_from_card(player_index, selected_card, &fields);
            let possible_spot: Option<(usize, Spot)> = if selected_card.rune().ability().is_swap() {
                display::select_spot_to_swap_card(
                    selected_card,
                    valid_spots,
                    &fields,
                    seasons.clone(),
                )
            } else {
                display::select_spot_to_play_card(selected_card, &valid_spots[0])
                    .map(|spot| (0, spot))
            };
            if let Some((field_index, spot_on_field)) = possible_spot {
                return Turn {
                    player_index,
                    field_index,
                    card_index_in_hand,
                    spot_on_field,
                };
            }
        }
    }
    /// Players draw their hands up to 10 cards and flip the top card
    /// of their decks to show their prize
    fn initialize_round(&mut self) {
        for player in &mut self.players {
            player.fill_hand();
            player.show_prize();
        }
    }
    /// Perform the play, removing the card from the player's hand and playing it
    /// in the correct location
    fn execute_turn(&mut self, turn: &Turn) {
        let card = self.players[turn.player_index].take_card_from_hand(turn.card_index_in_hand);
        let possible_other_card =
            self.players[turn.field_index].play_card(card, turn.spot_on_field);
        if let Some(other_card) = possible_other_card {
            self.players[turn.player_index].add_card_to_hand(other_card);
        }
    }
    /// Check first for a game-winning condition, then for a round-winning condition
    fn check_for_win_conditions(&self, turn: &Turn) -> Option<WinState> {
        let field_index = turn.field_index;
        let player_played_on = &self.players[field_index];
        let field = player_played_on.field();
        let spot = turn.spot_on_field;
        let player_season = player_played_on.season();
        let card = field
            .get(spot)
            .as_ref()
            .expect("Should be a card here from excuting turn");

        if card.season() == player_season {
            let field_in_season = field.clone_in_season(player_season);

            // If there is a win condition on the in-season field, then it is a game win
            let opt_win_cond = check_win(&field_in_season, &spot, card);
            if let Some(condition) = opt_win_cond {
                return Some(WinState {
                    player_index: field_index,
                    game_won: true,
                    condition,
                });
            }
        }

        let opt_win_cond = check_win(field, &spot, card);
        if let Some(condition) = opt_win_cond {
            let game_won = check_two_ancients_house_rule(&field.court, &condition, player_season);
            Some(WinState {
                player_index: field_index,
                game_won,
                condition,
            })
        } else {
            None
        }
    }
}
