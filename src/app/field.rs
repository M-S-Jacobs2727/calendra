use std::fmt::Display;
use strum_macros::Display;

use super::{card::Card, season::Season};

pub(crate) type RowOfCards = [Option<Card>; 5];

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Field {
    pub(crate) court: RowOfCards,
    pub(crate) garden: RowOfCards,
}
impl Field {
    pub(crate) fn new() -> Self {
        Self {
            court: [None, None, None, None, None],
            garden: [None, None, None, None, None],
        }
    }
    pub(crate) fn row(&self, row: Row) -> &RowOfCards {
        match row {
            Row::Court => &self.court,
            Row::Garden => &self.garden,
        }
    }
    pub(crate) fn set(&mut self, card: Option<Card>, spot: Spot) {
        match spot.row() {
            Row::Court => self.court[spot.place() as usize] = card,
            Row::Garden => self.garden[spot.place() as usize] = card,
        };
    }
    pub(crate) fn get(&self, spot: Spot) -> &Option<Card> {
        match spot.row() {
            Row::Court => &self.court[spot.place() as usize],
            Row::Garden => &self.garden[spot.place() as usize],
        }
    }
    /// Clone the field, keeping only the cards in the given season
    pub(crate) fn clone_in_season(&self, season: Season) -> Self {
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
    pub(super) fn iter<'a>(&'a self) -> FieldIter<'a> {
        FieldIter::new(self)
    }
}

pub(super) struct FieldIter<'a> {
    index: usize,
    field: &'a Field,
}
impl<'a> FieldIter<'a> {
    fn new(field: &'a Field) -> Self {
        Self { index: 0, field }
    }
}
impl<'a> Iterator for FieldIter<'a> {
    type Item = Option<Card>;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index <= 5 {
            Some(self.field.garden[self.index - 1])
        } else if self.index <= 10 {
            Some(self.field.court[self.index - 6])
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Display, PartialEq, Debug)]
pub(crate) enum Row {
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
pub(crate) struct Spot {
    row: Row,
    place: usize,
}
impl Spot {
    pub(crate) fn new(row: Row, place: usize) -> Self {
        assert!(place < 5);
        Self { row, place }
    }
    pub(crate) fn place(&self) -> usize {
        self.place
    }
    pub(crate) fn row(&self) -> &Row {
        &self.row
    }
    pub(crate) fn from_index(index: usize) -> Self {
        if index >= 10 {
            panic!("Index should be less than 10, found {}", index);
        }
        Self {
            row: if index < 5 { Row::Garden } else { Row::Court },
            place: index % 5,
        }
    }
}
impl Display for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{} {}", self.row, self.place + 1).fmt(f)
    }
}
