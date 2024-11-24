use strum_macros::Display;

use super::{Card, Field, Row, RowOfCards, RowScoreModifier, Rune, Score, Season, Spot};

#[derive(PartialEq, Debug, Display)]
pub(crate) enum WinCondition {
    CountCountess([Spot; 2]),
    ThreeInCourt([Spot; 3]),
    TwoPlagues([Spot; 2]),
    FourtyPoints,
}

pub(crate) fn check_win(field: &Field, spot: &Spot, card: &Card) -> Option<WinCondition> {
    if let Rune::Plague = card.rune() {
        return if let Some(spots) = check_two_plagues(field.row(spot.row().opposite()), spot) {
            Some(WinCondition::TwoPlagues([spots[0], spots[1]]))
        } else {
            None
        };
    }
    if *spot.row() == Row::Garden {
        return if check_fourty_points(field) {
            Some(WinCondition::FourtyPoints)
        } else {
            None
        };
    }
    if let Some(spots) = check_court(&field.court, card.rune()) {
        return if spots.len() == 2 {
            Some(WinCondition::CountCountess([spots[0], spots[1]]))
        } else {
            Some(WinCondition::ThreeInCourt([spots[0], spots[1], spots[2]]))
        };
    }
    if check_fourty_points(field) {
        Some(WinCondition::FourtyPoints)
    } else {
        None
    }
}

pub(crate) fn check_two_ancients_house_rule(
    row: &RowOfCards,
    condition: &WinCondition,
    season: Season,
) -> bool {
    // House rule: Two ancients, one in season and the other Ferric, count as a game win
    if let WinCondition::CountCountess([spot1, spot2]) = condition {
        let c1 = row[spot1.place()].expect("Should be a card here");
        let c2 = row[spot2.place()].expect("Should be a card here");
        if c1.rune() == Rune::Ancient
            && c2.rune() == Rune::Ancient
            && ((c1.season() == Season::Ferric && c2.season() == season)
                || (c2.season() == Season::Ferric && c1.season() == season))
        {
            return true;
        }
    }
    false
}

pub(crate) fn check_court(court: &[Option<Card>; 5], rune: Rune) -> Option<Vec<Spot>> {
    let num_cards_required = match rune {
        Rune::Ancient => 0,
        Rune::Beast | Rune::Changeling | Rune::Queen => 3,
        Rune::Count | Rune::Countess => 2,
        _ => return None,
    };
    // Now, the card was played in the Court and was one of
    // Ancient, Beast, Count/Countess, Changeling, or Queen

    let collected_court: Vec<&Card> = court.iter().filter_map(|o| o.as_ref()).collect();
    if collected_court.len() < num_cards_required {
        return None;
    }
    if rune == Rune::Ancient {
        // Check two ancients
        let ancient_spots: Vec<Spot> = collected_court
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| {
                if c.rune() == Rune::Ancient {
                    Some(Spot::new(Row::Court, i as usize))
                } else {
                    None
                }
            })
            .collect();
        assert!(ancient_spots.len() >= 1);
        if ancient_spots.len() == 2 {
            return Some(ancient_spots);
        }

        // Check Ancient and Count or Countess
        let count_pos = collected_court
            .iter()
            .position(|c| c.rune() == Rune::Count || c.rune() == Rune::Countess);
        if let Some(pos) = count_pos {
            return Some(vec![ancient_spots[0], Spot::new(Row::Court, pos as usize)]);
        }

        // Check Ancient and two Beasts, Changelings, or Queens
        for r in vec![Rune::Beast, Rune::Changeling, Rune::Queen] {
            let mut spots: Vec<Spot> = collected_court
                .iter()
                .enumerate()
                .filter_map(|(i, &c)| {
                    if c.rune() == r {
                        Some(Spot::new(Row::Court, i as usize))
                    } else {
                        None
                    }
                })
                .collect();
            if spots.len() == 2 {
                spots.push(ancient_spots[0]);
                return Some(spots);
            }
        }
    } else if num_cards_required == 2 {
        let positions: Vec<usize> = court
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if let Some(card) = c {
                    match card.rune() {
                        Rune::Ancient | Rune::Count | Rune::Countess => Some(i),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect();
        return if positions.len() < 2
            || court[positions[0]].unwrap().rune() == court[positions[1]].unwrap().rune()
        {
            None
        } else {
            Some(vec![
                Spot::new(Row::Court, positions[0]),
                Spot::new(Row::Court, positions[1]),
            ])
        };
    } else {
        let valid_spots: Vec<Spot> = court
            .iter()
            .enumerate()
            .filter_map(|(i, &card)| {
                if card.is_some_and(|c| c.rune() == rune || c.rune() == Rune::Ancient) {
                    Some(Spot::new(Row::Court, i as usize))
                } else {
                    None
                }
            })
            .collect();
        if valid_spots.len() >= 3 {
            return Some(valid_spots);
        }
    }
    None
}

pub(crate) fn check_two_plagues(row: &RowOfCards, spot: &Spot) -> Option<Vec<Spot>> {
    let spots: Vec<Spot> = row
        .iter()
        .enumerate()
        .filter_map(|(i, o)| match o {
            Some(c) => match c.rune() {
                Rune::Plague => Some(Spot::new(spot.row().opposite(), i as usize)),
                _ => None,
            },
            None => None,
        })
        .collect();

    if spots.len() == 0 {
        None
    } else {
        Some(vec![*spot, spots[0]])
    }
}

pub(crate) fn check_fourty_points(field: &Field) -> bool {
    count_points_in_row(&field.court, |c| c.court_score())
        + count_points_in_row(&field.garden, |c| c.garden_score())
        >= 40
}

fn count_points_in_row(row: &RowOfCards, card_score_fn: fn(&Card) -> Score) -> i32 {
    let mut scores = [0; 5];
    let num_cards = row.iter().filter(|&c| c.is_some()).count();
    let mut total = 0;

    for i in 0..5usize {
        if let Some(c) = row[i].as_ref() {
            if let Score::Value(v) = card_score_fn(c) {
                scores[i] = v;
            }
        }
    }
    for i in 0..5usize {
        if let Some(card) = row[i].as_ref() {
            if let Score::Mod(RowScoreModifier::Add(value)) = card_score_fn(card) {
                total += value * num_cards as i32;
            }
        }
    }

    for s in scores {
        total += s;
    }

    let multiplier = row
        .iter()
        .map(|card| match card {
            Some(card) => match card_score_fn(card) {
                Score::Mod(RowScoreModifier::Mult(value)) => value,
                _ => 1,
            },
            None => 1,
        })
        .reduce(|acc, el| acc * el)
        .unwrap();

    total * multiplier
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_plagues_meets_win_condition() {
        let mut field = Field::new();
        let card = Card::create_plague(Season::Autumn);

        let spot = Spot::new(Row::Court, 0);
        field.set(Some(card.clone()), spot.clone());
        field.set(Some(card), Spot::new(Row::Garden, 1));
        let wc = check_win(&field, &spot, &card);

        assert!(wc.is_some(), "Should be a win condition");
        assert!(
            match wc.unwrap() {
                WinCondition::TwoPlagues(_) => true,
                _ => false,
            },
            "Should be two plagues"
        );
    }

    #[test]
    fn test_two_countesses_gives_no_win_condition() {
        let card1 = Card::create_countess(Season::Autumn);
        let card2 = Card::create_countess(Season::Ferric);
        let spot1 = Spot::new(Row::Court, 0);
        let spot2 = Spot::new(Row::Court, 1);

        let mut field = Field::new();
        field.set(Some(card1), spot1);
        field.set(Some(card2), spot2);

        let wc = check_win(&field, &spot1, &card1);
        assert!(wc.is_none());
    }

    #[test]
    fn test_two_counts_gives_no_win_condition() {
        let card1 = Card::create_count(Season::Autumn);
        let card2 = Card::create_count(Season::Ferric);
        let spot1 = Spot::new(Row::Court, 0);
        let spot2 = Spot::new(Row::Court, 1);

        let mut field = Field::new();
        field.set(Some(card1), spot1);
        field.set(Some(card2), spot2);

        let wc = check_win(&field, &spot1, &card1);
        assert!(wc.is_none());
    }

    #[test]
    fn test_count_and_countess_gives_win_condition() {
        let card1 = Card::create_count(Season::Autumn);
        let card2 = Card::create_countess(Season::Ferric);
        let spot1 = Spot::new(Row::Court, 0);
        let spot2 = Spot::new(Row::Court, 1);

        let mut field = Field::new();
        field.set(Some(card1), spot1);
        field.set(Some(card2), spot2);

        let wc = check_win(&field, &spot1, &card1);
        assert!(wc.is_some());
        assert!(match wc.unwrap() {
            WinCondition::CountCountess(_) => true,
            _ => false,
        });
    }
}