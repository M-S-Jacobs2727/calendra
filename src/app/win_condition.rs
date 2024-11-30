use strum_macros::Display;

use super::{
    card::{ability::Ability, rune::Rune, score::Score, Card},
    field::{Field, Row, RowOfCards, Spot},
    season::Season,
};

#[derive(PartialEq, Debug, Display)]
pub(crate) enum WinCondition {
    CountCountess([Spot; 2]),
    ThreeInCourt([Spot; 3]),
    TwoPlagues([Spot; 2]),
    FourtyPoints,
}

/// After a card is played in a spot on a player's field, check if that field now
/// counts as a win condition for that player.
pub(crate) fn check_win(field: &Field, spot: &Spot, card: &Card) -> Option<WinCondition> {
    // A Plague card can only count as a win for the TwoPlagues win condition
    if let Rune::Plague = card.rune() {
        return if let Some(spots) = check_two_plagues(field.row(spot.row().opposite()), spot) {
            Some(WinCondition::TwoPlagues([spots[0], spots[1]]))
        } else {
            None
        };
    }
    // After accounting for the TwoPlagues win condition, if the card was played in the
    // Garden, then only the FourtyPoints win condition is possible
    if *spot.row() == Row::Garden {
        return if check_fourty_points(field) {
            Some(WinCondition::FourtyPoints)
        } else {
            None
        };
    }
    // check_court short-circuits for non-applicable runes, so this is faster than check_fourty_points
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

/// House rule: Two ancients in the court, one in season and the other Ferric,
/// counts as a game win
pub(crate) fn check_two_ancients_house_rule(
    court: &RowOfCards,
    condition: &WinCondition,
    season: Season,
) -> bool {
    if let WinCondition::CountCountess([spot1, spot2]) = condition {
        let c1 = court[spot1.place()].expect("Should be a card here");
        let c2 = court[spot2.place()].expect("Should be a card here");
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
/// Check the court if the rune of the played card is Ancient, Beast, Changeling, Queen,
/// Count, or Countess. No other runes are possible for this win condition.
fn check_court(court: &[Option<Card>; 5], rune: Rune) -> Option<Vec<Spot>> {
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
        // Checking for Count and Countess, Count and Ancient, or Countess and Ancient
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

        // If the runes are the same (e.g., Count and Count) then this is not a win condition
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
        // Check for three cards of the same rune as the card played, or an Ancient
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

/// Check the TwoPlagues win condition, where the Plagues must be played in
/// opposite rows (one in the Court, one in the Garden).
fn check_two_plagues(row: &RowOfCards, spot: &Spot) -> Option<Vec<Spot>> {
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

/// Check if the sum of cards on the field are at least 40
fn check_fourty_points(field: &Field) -> bool {
    count_points_in_row(&field.court, |c| c.court_score())
        + count_points_in_row(&field.garden, |c| c.garden_score())
        >= 40
}

/// Counts the total number of points in a row in the following order:
/// 1. Count the individual points on each card
/// 2. Count the Adj +1 modifiers from Counts/Countesses
/// 3. Count the Row -1 modifiers from Mists
/// 4. Count the Row xN modifiers from Weathers and Plagues
fn count_points_in_row(row: &RowOfCards, card_score_fn: fn(&Card) -> Score) -> i32 {
    let count_countess_locations: Vec<usize> = (0..5)
        .filter(|i| {
            row[*i].is_some_and(|c| match c.rune() {
                Rune::Count | Rune::Countess => true,
                _ => false,
            })
        })
        .collect();
    let num_mists = row
        .iter()
        .filter(|c| {
            c.is_some_and(|card| match card.rune() {
                Rune::Mist => true,
                _ => false,
            })
        })
        .count();
    let num_weathers = row
        .iter()
        .filter(|c| {
            c.is_some_and(|card| match card.rune() {
                Rune::Weather => true,
                _ => false,
            })
        })
        .count();
    let num_plagues = row
        .iter()
        .filter(|c| {
            c.is_some_and(|card| match card.rune() {
                Rune::Plague => true,
                _ => false,
            })
        })
        .count();

    row.iter()
        .enumerate()
        .map(|(i, possible_card)| {
            if let Some(card) = possible_card {
                if let Score::Value(mut value) = card_score_fn(card) {
                    value -= num_mists as i32;
                    if count_countess_locations.contains(&(i + 1)) {
                        value += 1;
                    }
                    if count_countess_locations.contains(&(i - 1)) {
                        value += 1;
                    }
                    if card.rune().ability() != Ability::NoWeather && num_weathers > 0 {
                        value *= num_weathers as i32;
                    }
                    if card.rune().ability() != Ability::AntiPlague && num_plagues > 0 {
                        value = 0;
                    }
                    return value;
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_field(garden_cards: [Option<Card>; 5], court_cards: [Option<Card>; 5]) -> Field {
        let mut field = Field::new();
        for i in 0..5usize {
            if let Some(card) = garden_cards[i] {
                field.set(Some(card), Spot::new(Row::Garden, i));
            }
        }
        for i in 0..5usize {
            if let Some(card) = court_cards[i] {
                field.set(Some(card), Spot::new(Row::Court, i));
            }
        }
        field
    }

    #[test]
    fn test_two_plagues_meets_win_condition() {
        let field = setup_field(
            [
                Some(Card::create_plague(Season::Autumn)),
                None,
                None,
                None,
                None,
            ],
            [
                Some(Card::create_plague(Season::Ferric)),
                None,
                None,
                None,
                None,
            ],
        );
        let spot = Spot::new(Row::Garden, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some(), "Should be a win condition");
        assert!(
            match win_condition.unwrap() {
                WinCondition::TwoPlagues(_) => true,
                _ => false,
            },
            "Should be two plagues"
        );
    }

    #[test]
    fn test_two_countesses_gives_no_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_countess(Season::Autumn)),
                Some(Card::create_countess(Season::Ferric)),
                None,
                None,
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());
        assert!(win_condition.is_none());
    }

    #[test]
    fn test_two_counts_gives_no_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_count(Season::Autumn)),
                Some(Card::create_count(Season::Ferric)),
                None,
                None,
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());
        assert!(win_condition.is_none());
    }

    #[test]
    fn test_count_and_countess_gives_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_count(Season::Autumn)),
                Some(Card::create_countess(Season::Ferric)),
                None,
                None,
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some());
        assert!(match win_condition.unwrap() {
            WinCondition::CountCountess(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_countess_and_ancient_gives_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_countess(Season::Autumn)),
                Some(Card::create_ancient(Season::Ferric)),
                None,
                None,
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some());
        assert!(match win_condition.unwrap() {
            WinCondition::CountCountess(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_count_and_ancient_gives_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_count(Season::Autumn)),
                Some(Card::create_ancient(Season::Ferric)),
                None,
                None,
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some());
        assert!(match win_condition.unwrap() {
            WinCondition::CountCountess(_) => true,
            _ => false,
        });
    }
    #[test]
    fn test_count_and_ancient_in_different_rows_gives_no_win_condition() {
        let field = setup_field(
            [
                None,
                None,
                None,
                None,
                Some(Card::create_count(Season::Autumn)),
            ],
            [
                Some(Card::create_ancient(Season::Ferric)),
                None,
                None,
                None,
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_none());
    }

    #[test]
    fn test_two_ancients_gives_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_ancient(Season::Autumn)),
                Some(Card::create_ancient(Season::Ferric)),
                None,
                None,
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some());
        assert!(match win_condition.unwrap() {
            WinCondition::CountCountess(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_three_queens_in_court_gives_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_queen(Season::Spring, 7)),
                None,
                Some(Card::create_queen(Season::Spring, 5)),
                Some(Card::create_queen(Season::Ferric, 9)),
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some());
        assert!(match win_condition.unwrap() {
            WinCondition::ThreeInCourt(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_three_beasts_in_court_gives_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_beast(Season::Spring, 10)),
                None,
                Some(Card::create_beast(Season::Spring, 12)),
                Some(Card::create_beast(Season::Ferric, 10)),
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some());
        assert!(match win_condition.unwrap() {
            WinCondition::ThreeInCourt(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_three_changelings_in_court_gives_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_changeling(Season::Spring)),
                None,
                Some(Card::create_changeling(Season::Spring)),
                Some(Card::create_changeling(Season::Ferric)),
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some());
        assert!(match win_condition.unwrap() {
            WinCondition::ThreeInCourt(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_one_ancient_and_two_queens_in_court_gives_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_queen(Season::Spring, 7)),
                None,
                Some(Card::create_ancient(Season::Spring)),
                Some(Card::create_queen(Season::Ferric, 9)),
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some());
        assert!(match win_condition.unwrap() {
            WinCondition::ThreeInCourt(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_one_ancient_and_two_beasts_in_court_gives_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_beast(Season::Spring, 10)),
                None,
                Some(Card::create_beast(Season::Spring, 12)),
                Some(Card::create_ancient(Season::Ferric)),
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some());
        assert!(match win_condition.unwrap() {
            WinCondition::ThreeInCourt(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_one_ancient_and_two_changelings_in_court_gives_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_ancient(Season::Spring)),
                None,
                Some(Card::create_changeling(Season::Spring)),
                Some(Card::create_changeling(Season::Ferric)),
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some());
        assert!(match win_condition.unwrap() {
            WinCondition::ThreeInCourt(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_two_ancients_and_one_queen_in_court_gives_count_countess_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_queen(Season::Spring, 7)),
                None,
                Some(Card::create_ancient(Season::Spring)),
                Some(Card::create_ancient(Season::Ferric)),
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 3);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some());
        assert!(match win_condition.unwrap() {
            WinCondition::CountCountess(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_fourty_points_only_mundane() {
        let field = setup_field(
            [
                Some(Card::create_beast(Season::Autumn, 12)),
                Some(Card::create_beast(Season::Winter, 12)),
                Some(Card::create_beast(Season::Spring, 12)),
                Some(Card::create_archer(Season::Ferric, 8)),
                None,
            ],
            [None; 5],
        );
        let spot = Spot::new(Row::Garden, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_some());
        assert!(match win_condition.unwrap() {
            WinCondition::FourtyPoints => true,
            _ => false,
        });
    }

    #[test]
    fn test_two_beasts_and_two_archers_in_court_give_no_win_condition() {
        let field = setup_field(
            [None; 5],
            [
                Some(Card::create_beast(Season::Autumn, 12)),
                Some(Card::create_beast(Season::Winter, 12)),
                Some(Card::create_archer(Season::Spring, 6)),
                Some(Card::create_archer(Season::Ferric, 8)),
                None,
            ],
        );
        let spot = Spot::new(Row::Court, 0);
        let win_condition = check_win(&field, &spot, field.get(spot).as_ref().unwrap());

        assert!(win_condition.is_none());

        let points = count_points_in_row(
            &[
                Some(Card::create_beast(Season::Autumn, 12)),
                Some(Card::create_beast(Season::Winter, 12)),
                Some(Card::create_archer(Season::Spring, 6)),
                Some(Card::create_archer(Season::Ferric, 8)),
                None,
            ],
            |c| c.court_score(),
        );
        assert_eq!(14, points);
    }
}
