pub(crate) mod ability;
pub(crate) mod rune;
pub(crate) mod score;

use ability::*;
use rune::*;
use score::*;

use std::fmt::Display;

use super::season::Season;

pub(crate) fn all_cards() -> Vec<Card> {
    vec![
        // Ferric
        Card::create_ancient(Season::Ferric),
        Card::create_archer(Season::Ferric, 8),
        Card::create_archer(Season::Ferric, 7),
        Card::create_beast(Season::Ferric, 10),
        Card::create_beast(Season::Ferric, 9),
        Card::create_changeling(Season::Ferric),
        Card::create_changeling(Season::Ferric),
        Card::create_count(Season::Ferric),
        Card::create_countess(Season::Ferric),
        Card::create_magician(Season::Ferric, 10),
        Card::create_magician(Season::Ferric, 9),
        Card::create_mist(),
        Card::create_mist(),
        Card::create_plague(Season::Ferric),
        Card::create_plague(Season::Ferric),
        Card::create_queen(Season::Ferric, 9),
        Card::create_queen(Season::Ferric, 7),
        Card::create_queen(Season::Ferric, 5),
        Card::create_warrior(Season::Ferric, 10),
        Card::create_warrior(Season::Ferric, 9),
        // Spring
        Card::create_ancient(Season::Spring),
        Card::create_archer(Season::Spring, 6),
        Card::create_archer(Season::Spring, 5),
        Card::create_archer(Season::Spring, 4),
        Card::create_beast(Season::Spring, 12),
        Card::create_beast(Season::Spring, 10),
        Card::create_beast(Season::Spring, 8),
        Card::create_changeling(Season::Spring),
        Card::create_changeling(Season::Spring),
        Card::create_changeling(Season::Spring),
        Card::create_changeling(Season::Spring),
        Card::create_count(Season::Spring),
        Card::create_countess(Season::Spring),
        Card::create_magician(Season::Spring, 9),
        Card::create_magician(Season::Spring, 8),
        Card::create_magician(Season::Spring, 7),
        Card::create_plague(Season::Spring),
        Card::create_plague(Season::Spring),
        Card::create_queen(Season::Spring, 7),
        Card::create_queen(Season::Spring, 5),
        Card::create_queen(Season::Spring, 3),
        Card::create_warrior(Season::Spring, 9),
        Card::create_warrior(Season::Spring, 8),
        Card::create_warrior(Season::Spring, 7),
        Card::create_weather(Season::Spring),
        // Summer
        Card::create_ancient(Season::Summer),
        Card::create_archer(Season::Summer, 6),
        Card::create_archer(Season::Summer, 5),
        Card::create_archer(Season::Summer, 4),
        Card::create_beast(Season::Summer, 12),
        Card::create_beast(Season::Summer, 10),
        Card::create_beast(Season::Summer, 8),
        Card::create_changeling(Season::Summer),
        Card::create_changeling(Season::Summer),
        Card::create_changeling(Season::Summer),
        Card::create_changeling(Season::Summer),
        Card::create_count(Season::Summer),
        Card::create_countess(Season::Summer),
        Card::create_magician(Season::Summer, 9),
        Card::create_magician(Season::Summer, 8),
        Card::create_magician(Season::Summer, 7),
        Card::create_plague(Season::Summer),
        Card::create_plague(Season::Summer),
        Card::create_queen(Season::Summer, 7),
        Card::create_queen(Season::Summer, 5),
        Card::create_queen(Season::Summer, 3),
        Card::create_warrior(Season::Summer, 9),
        Card::create_warrior(Season::Summer, 8),
        Card::create_warrior(Season::Summer, 7),
        Card::create_weather(Season::Summer),
        // Autumn
        Card::create_ancient(Season::Autumn),
        Card::create_archer(Season::Autumn, 6),
        Card::create_archer(Season::Autumn, 5),
        Card::create_archer(Season::Autumn, 4),
        Card::create_beast(Season::Autumn, 12),
        Card::create_beast(Season::Autumn, 10),
        Card::create_beast(Season::Autumn, 8),
        Card::create_changeling(Season::Autumn),
        Card::create_changeling(Season::Autumn),
        Card::create_changeling(Season::Autumn),
        Card::create_changeling(Season::Autumn),
        Card::create_count(Season::Autumn),
        Card::create_countess(Season::Autumn),
        Card::create_magician(Season::Autumn, 9),
        Card::create_magician(Season::Autumn, 8),
        Card::create_magician(Season::Autumn, 7),
        Card::create_plague(Season::Autumn),
        Card::create_plague(Season::Autumn),
        Card::create_queen(Season::Autumn, 7),
        Card::create_queen(Season::Autumn, 5),
        Card::create_queen(Season::Autumn, 3),
        Card::create_warrior(Season::Autumn, 9),
        Card::create_warrior(Season::Autumn, 8),
        Card::create_warrior(Season::Autumn, 7),
        Card::create_weather(Season::Autumn),
        // Winter
        Card::create_ancient(Season::Winter),
        Card::create_archer(Season::Winter, 6),
        Card::create_archer(Season::Winter, 5),
        Card::create_archer(Season::Winter, 4),
        Card::create_beast(Season::Winter, 12),
        Card::create_beast(Season::Winter, 10),
        Card::create_beast(Season::Winter, 8),
        Card::create_changeling(Season::Winter),
        Card::create_changeling(Season::Winter),
        Card::create_changeling(Season::Winter),
        Card::create_changeling(Season::Winter),
        Card::create_count(Season::Winter),
        Card::create_countess(Season::Winter),
        Card::create_magician(Season::Winter, 9),
        Card::create_magician(Season::Winter, 8),
        Card::create_magician(Season::Winter, 7),
        Card::create_plague(Season::Winter),
        Card::create_plague(Season::Winter),
        Card::create_queen(Season::Winter, 7),
        Card::create_queen(Season::Winter, 5),
        Card::create_queen(Season::Winter, 3),
        Card::create_warrior(Season::Winter, 9),
        Card::create_warrior(Season::Winter, 8),
        Card::create_warrior(Season::Winter, 7),
        Card::create_weather(Season::Winter),
    ]
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Card {
    season: Season,
    rune: Rune,
    court_score: Score,
    garden_score: Score,
}
impl Card {
    pub(crate) fn create_ancient(season: Season) -> Self {
        let score = match season {
            Season::Ferric => 12,
            _ => 10,
        };
        let score = Score::Value(score);
        Self {
            season,
            rune: Rune::Ancient,
            garden_score: score,
            court_score: score,
        }
    }
    pub(crate) fn create_archer(season: Season, score: i32) -> Self {
        match season {
            Season::Ferric => assert!(score == 7 || score == 8),
            _ => assert!(score < 7 && score > 4),
        };
        let score = Score::Value(score);
        Self {
            season,
            rune: Rune::Archer,
            garden_score: score,
            court_score: score,
        }
    }
    pub(crate) fn create_beast(season: Season, garden_score: i32) -> Self {
        let court_score = match season {
            Season::Ferric => {
                assert!(garden_score == 10 || garden_score == 9);
                garden_score
            }
            _ => {
                assert!(garden_score == 12 || garden_score == 10 || garden_score == 8);
                0
            }
        };
        let garden_score = Score::Value(garden_score);
        let court_score = Score::Value(court_score);
        Self {
            season,
            rune: Rune::Beast,
            garden_score,
            court_score,
        }
    }
    pub(crate) fn create_changeling(season: Season) -> Self {
        let score = match season {
            Season::Ferric => 2,
            _ => 0,
        };
        let score = Score::Value(score);
        Self {
            season,
            rune: Rune::Changeling,
            court_score: score,
            garden_score: score,
        }
    }
    pub(crate) fn create_count(season: Season) -> Self {
        let score = match season {
            Season::Ferric => 9,
            _ => 8,
        };
        let score = Score::Value(score);
        Self {
            season,
            rune: Rune::Count,
            court_score: score,
            garden_score: score,
        }
    }
    pub(crate) fn create_countess(season: Season) -> Self {
        let score = match season {
            Season::Ferric => 10,
            _ => 9,
        };
        let score = Score::Value(score);
        Self {
            season,
            rune: Rune::Countess,
            court_score: score,
            garden_score: score,
        }
    }
    pub(crate) fn create_magician(season: Season, court_score: i32) -> Self {
        let garden_score = match season {
            Season::Ferric => {
                assert!(court_score == 10 || court_score == 9);
                10 - court_score
            }
            _ => {
                assert!(court_score == 9 || court_score == 8 || court_score == 7);
                10 - court_score
            }
        };
        let garden_score = Score::Value(garden_score);
        let court_score = Score::Value(court_score);
        Self {
            season,
            rune: Rune::Magician,
            garden_score,
            court_score,
        }
    }
    pub(crate) fn create_mist() -> Self {
        Self {
            season: Season::Ferric,
            rune: Rune::Mist,
            garden_score: Score::Mod(RowScoreModifier::Add(-1)),
            court_score: Score::Mod(RowScoreModifier::Add(-1)),
        }
    }
    pub(crate) fn create_plague(season: Season) -> Self {
        Self {
            season,
            rune: Rune::Plague,
            garden_score: Score::Mod(RowScoreModifier::Mult(0)),
            court_score: Score::Mod(RowScoreModifier::Mult(0)),
        }
    }
    pub(crate) fn create_queen(season: Season, score: i32) -> Self {
        match season {
            Season::Ferric => assert!(score == 9 || score == 7 || score == 5),
            _ => assert!(score == 7 || score == 5 || score == 3),
        };
        let score = Score::Value(score);
        Self {
            season,
            rune: Rune::Queen,
            garden_score: score,
            court_score: score,
        }
    }
    pub(crate) fn create_warrior(season: Season, garden_score: i32) -> Self {
        let court_score = match season {
            Season::Ferric => {
                assert!(garden_score == 10 || garden_score == 9);
                10 - garden_score
            }
            _ => {
                assert!(garden_score == 9 || garden_score == 8 || garden_score == 7);
                10 - garden_score
            }
        };
        let garden_score = Score::Value(garden_score);
        let court_score = Score::Value(court_score);
        Self {
            season,
            rune: Rune::Warrior,
            garden_score,
            court_score,
        }
    }
    pub(crate) fn create_weather(season: Season) -> Self {
        let score = Score::Mod(RowScoreModifier::Mult(2));
        Self {
            season,
            rune: Rune::Weather,
            court_score: score,
            garden_score: score,
        }
    }

    pub(crate) fn season(&self) -> Season {
        self.season
    }
    pub(crate) fn rune(&self) -> Rune {
        self.rune
    }
    pub(crate) fn court_score(&self) -> Score {
        self.court_score
    }
    pub(crate) fn garden_score(&self) -> Score {
        self.garden_score
    }

    pub(crate) fn to_text(&self) -> String {
        format!(
            "{} {} {}/{}",
            self.season, self.rune, self.garden_score, self.court_score
        )
    }

    pub(crate) fn can_swap_with(&self, card: &Card) -> bool {
        match (self.rune, card.rune.ability()) {
            (Rune::Mist, _) => true,
            (Rune::Changeling, Ability::AntiSwap) => false,
            (Rune::Changeling, _) => true,
            (Rune::Plague, Ability::AntiSwap | Ability::AntiPlague) => false,
            (Rune::Plague, _) => true,
            (_, _) => false,
        }
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.garden_score, self.court_score) {
            (Score::Value(gs), Score::Value(cs)) => {
                format!("{}\n{}\n{}\n{}", self.season, self.rune, gs, cs).fmt(f)
            }
            _ => format!("{}\n{}", self.season, self.rune).fmt(f),
        }
    }
}
