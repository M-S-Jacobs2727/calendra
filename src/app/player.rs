use rand::seq::SliceRandom;

use super::{
    card::Card,
    field::{Field, Row, Spot},
    Season,
};

pub type Deck = Vec<Card>;
pub type Hand = Vec<Card>;

pub struct Player {
    deck: Deck,
    hand: Hand,
    prize: Option<Card>,
    field: Field,
    season: Season,
}
impl Player {
    pub fn new(season: Season) -> Self {
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
    // pub fn deck(&self) -> &Vec<Card> {
    //     &self.deck
    // }
    pub fn hand(&self) -> &Vec<Card> {
        &self.hand
    }
    pub fn prize(&self) -> &Option<Card> {
        &self.prize
    }
    pub fn field(&self) -> &Field {
        &self.field
    }
    pub(crate) fn season(&self) -> Season {
        self.season
    }

    // Setters
    pub fn set_deck(&mut self, deck: Vec<Card>) {
        self.deck = deck;
    }
    pub fn take_prize(&mut self) -> Card {
        self.prize.take().expect("Expected a prize")
    }

    // Actions
    pub fn fill_hand(&mut self) {
        let num_cards_to_draw = 10 - self.hand.len();
        let at = self.deck.len() - num_cards_to_draw;
        self.hand.append(&mut self.deck.split_off(at));
    }
    pub fn show_prize(&mut self) {
        self.prize = self.deck.pop();
    }
    pub fn move_hand_to_deck(&mut self) {
        self.deck.append(&mut self.hand);
    }
    pub fn reset_field(&mut self) {
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
    pub fn add_to_deck(&mut self, card: Card) {
        self.deck.push(card);
    }
    pub fn add_to_hand(&mut self, card: Card) {
        self.hand.push(card);
    }
    pub fn swap_prizes(&mut self, other: &mut Self) {
        let prize = self.take_prize();
        self.prize = Some(other.take_prize());
        other.prize = Some(prize);
    }
    pub(crate) fn shuffle_deck(&mut self) {
        self.deck.shuffle(&mut rand::thread_rng());
    }
    pub(crate) fn take_from_hand(&mut self, card_index: usize) -> Card {
        self.hand.remove(card_index)
    }
    pub(crate) fn play_card(&mut self, card: Card, spot: Spot) -> Option<Card> {
        let old_card = match spot.row() {
            Row::Garden => self.field.garden[spot.place()].take(),
            Row::Court => self.field.court[spot.place()].take(),
        };
        self.field.set(Some(card), spot);
        old_card
    }
}
