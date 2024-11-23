use super::{card::Card, Season};

pub type RowOfCards = [Option<Card>; 5];

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Field {
    pub court: RowOfCards,
    pub garden: RowOfCards,
}
impl Field {
    pub fn new() -> Self {
        Self {
            court: [None, None, None, None, None],
            garden: [None, None, None, None, None],
        }
    }
    pub fn row(&self, row: Row) -> &RowOfCards {
        match row {
            Row::Court => &self.court,
            Row::Garden => &self.garden,
        }
    }
    pub fn set(&mut self, card: Option<Card>, spot: Spot) {
        match spot.row() {
            Row::Court => self.court[spot.place() as usize] = card,
            Row::Garden => self.garden[spot.place() as usize] = card,
        };
    }
    pub fn get(&self, spot: Spot) -> &Option<Card> {
        match spot.row() {
            Row::Court => &self.court[spot.place() as usize],
            Row::Garden => &self.garden[spot.place() as usize],
        }
    }
    pub fn clone_in_season(&self, season: Season) -> Self {
        let mut field_in_season = self.clone();
        for row in [Row::Garden, Row::Court] {
            for place in 0..5usize {
                let c = field_in_season.row(row)[place];
                if c.is_some_and(|card| card.season() != season) {
                    field_in_season.set(None, Spot::new(row, place));
                }
            }
        }
        field_in_season
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Row {
    Garden,
    Court,
}
impl Row {
    pub(crate) fn opposite(&self) -> Self {
        match self {
            Row::Court => Row::Garden,
            Row::Garden => Row::Court,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Spot {
    row: Row,
    place: usize,
}
impl Spot {
    pub fn new(row: Row, place: usize) -> Self {
        assert!(place < 5);
        Self { row, place }
    }
    pub fn place(&self) -> usize {
        self.place
    }
    pub fn row(&self) -> &Row {
        &self.row
    }
}
