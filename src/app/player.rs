use rand::seq::SliceRandom;

use super::{
    card::Card,
    field::{Field, Row, Spot},
    season::Season,
};

pub(crate) type Deck = Vec<Card>;
pub(crate) type Hand = Vec<Card>;

pub(crate) struct Player {
    deck: Deck,
    hand: Hand,
    prize: Option<Card>,
    field: Field,
    season: Season,
}
impl Player {
    pub(crate) fn new(season: Season) -> Self {
        assert!(
            season != Season::Ferric,
            "Player's season should not be Ferric"
        );
        Self {
            deck: vec![],
            hand: vec![],
            prize: None,
            field: Field::new(),
            season,
        }
    }

    // Getters
    pub(crate) fn hand(&self) -> &Vec<Card> {
        &self.hand
    }
    pub(crate) fn prize(&self) -> &Option<Card> {
        &self.prize
    }
    pub(crate) fn field(&self) -> &Field {
        &self.field
    }
    pub(crate) fn season(&self) -> Season {
        self.season
    }

    // Setters
    pub(crate) fn set_deck(&mut self, deck: Vec<Card>) {
        self.deck = deck;
    }
    pub(crate) fn take_prize(&mut self) -> Card {
        self.prize.take().expect("Expected a prize")
    }

    // Actions
    /// Draw from the deck until the hand has 10 cards
    pub(crate) fn fill_hand(&mut self) {
        let num_cards_to_draw = 10 - self.hand.len();
        let at = self.deck.len() - num_cards_to_draw;
        self.hand.append(&mut self.deck.split_off(at));
    }
    /// Flip the top card of the deck to show the prize
    pub(crate) fn show_prize(&mut self) {
        self.prize = self.deck.pop();
    }
    /// Discard the hand into the deck (at the end of a round)
    pub(crate) fn move_hand_to_deck(&mut self) {
        self.deck.append(&mut self.hand);
    }
    /// Send all cards that are in season to the hand, all others to the deck
    pub(crate) fn remove_cards_from_field(&mut self) {
        for i in 0..5usize {
            let opt_card = self.field.court[i].take();
            if let Some(card) = opt_card {
                if card.season() == self.season {
                    self.hand.push(card);
                } else {
                    self.deck.push(card);
                }
            }
        }
        for i in 0..5usize {
            let opt_card = self.field.garden[i].take();
            if let Some(card) = opt_card {
                if card.season() == self.season {
                    self.hand.push(card);
                } else {
                    self.deck.push(card);
                }
            }
        }
    }
    pub(crate) fn add_card_to_deck(&mut self, card: Card) {
        self.deck.push(card);
    }
    pub(crate) fn add_card_to_hand(&mut self, card: Card) {
        self.hand.push(card);
    }
    pub(crate) fn shuffle_deck(&mut self) {
        self.deck.shuffle(&mut rand::thread_rng());
    }
    pub(crate) fn take_card_from_hand(&mut self, card_index: usize) -> Card {
        self.hand.remove(card_index)
    }
    /// Set a card in the given spot on the player's field. Return the card
    /// that was in that spot previously, if any.
    pub(crate) fn play_card(&mut self, card: Card, spot: Spot) -> Option<Card> {
        let old_card = match spot.row() {
            Row::Garden => self.field.garden[spot.place()].take(),
            Row::Court => self.field.court[spot.place()].take(),
        };
        self.field.set(Some(card), spot);
        old_card
    }
    /// Set a card as the prize. (Used when swapping at the end of a round).
    pub(crate) fn set_prize(&mut self, prize: Card) -> Option<Card> {
        let current_prize = self.prize.take();
        self.prize = Some(prize);
        current_prize
    }
}
