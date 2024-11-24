mod ability;
mod rune;
mod score;

pub(crate) use ability::*;
pub(crate) use rune::*;
pub(crate) use score::*;

use std::{
    fmt::Display,
    fs,
    io::{self, BufRead},
    path::Path,
};

use super::Season;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
            rune: Rune::Beast,
            garden_score,
            court_score,
        }
    }
    pub(crate) fn create_mist() -> Self {
        Self {
            season: Season::Ferric,
            rune: Rune::Mist,
            garden_score: Score::Mod(ScoreModifier {
                op: Op::Add(-1),
                affected: AffectedCards::Row,
            }),
            court_score: Score::Mod(ScoreModifier {
                op: Op::Add(-1),
                affected: AffectedCards::Row,
            }),
        }
    }
    pub(crate) fn create_plague(season: Season) -> Self {
        Self {
            season,
            rune: Rune::Plague,
            garden_score: Score::Mod(ScoreModifier {
                op: Op::Mult(0),
                affected: AffectedCards::Row,
            }),
            court_score: Score::Mod(ScoreModifier {
                op: Op::Mult(0),
                affected: AffectedCards::Row,
            }),
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
            rune: Rune::Beast,
            garden_score,
            court_score,
        }
    }
    pub(crate) fn create_weather(season: Season) -> Self {
        let score = Score::Mod(ScoreModifier {
            op: Op::Mult(2),
            affected: AffectedCards::Row,
        });
        Self {
            season,
            rune: Rune::Weather,
            court_score: score,
            garden_score: score,
        }
    }

    pub(crate) fn load_all(filename: &str) -> Vec<Self> {
        match read_lines(filename) {
            Ok(lines) => lines
                .skip(1)
                .map(|line| Card::from_csv_row(line.expect("Failed to read row")))
                .collect(),
            Err(_) => panic!("Failed to read file"),
        }
    }
    fn from_csv_row(row: String) -> Self {
        let words: Vec<&str> = row.split(',').collect();
        assert_eq!(
            4,
            words.len(),
            "Row should have four fields: Season,Rune,CourtScore,GardenScore"
        );
        let season = match words[0] {
            "Ferric" => Season::Ferric,
            "Spring" => Season::Spring,
            "Summer" => Season::Summer,
            "Autumn" => Season::Autumn,
            "Winter" => Season::Winter,
            _ => panic!("Invalid Season string: {}", words[0]),
        };
        let rune = match words[1] {
            "Ancient" => Rune::Ancient,
            "Archer" => Rune::Archer,
            "Beast" => Rune::Beast,
            "Changeling" => Rune::Changeling,
            "Count" => Rune::Count,
            "Countess" => Rune::Countess,
            "Magician" => Rune::Magician,
            "Mist" => Rune::Mist,
            "Plague" => Rune::Plague,
            "Queen" => Rune::Queen,
            "Warrior" => Rune::Warrior,
            "Weather" => Rune::Weather,
            _ => panic!("Invalid Rune string: {}", words[1]),
        };
        let garden_score = Score::from_string(words[2]);
        let court_score = Score::from_string(words[3]);
        Self {
            season,
            rune,
            court_score,
            garden_score,
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
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(
            "{}\n{}\n{}\n{}",
            self.season, self.rune, self.garden_score, self.court_score
        )
        .fmt(f)
    }
}
impl From<String> for Card {
    fn from(value: String) -> Self {
        let words: Vec<&str> = value.split(' ').collect();
        assert!(words.len() == 3 || words.len() == 2);
        let season: Season = words[0].into();
        let rune: Rune = words[1].into();
        let (garden_score, court_score) = if words.len() == 3 {
            let scores: Vec<&str> = words[2].split('/').collect();
            assert_eq!(scores.len(), 2, "Should be two scores (garden and court)");
            (scores[0].into(), scores[1].into())
        } else {
            let is_ferric = season == Season::Ferric;
            match rune {
                Rune::Ancient => {
                    if is_ferric {
                        (Score::Value(12), Score::Value(12))
                    } else {
                        (Score::Value(10), Score::Value(10))
                    }
                }
                Rune::Changeling => {
                    if is_ferric {
                        (Score::Value(2), Score::Value(2))
                    } else {
                        (Score::Value(1), Score::Value(1))
                    }
                }
                Rune::Count => {
                    if is_ferric {
                        (Score::Value(9), Score::Value(9))
                    } else {
                        (Score::Value(8), Score::Value(8))
                    }
                }
                Rune::Countess => {
                    if is_ferric {
                        (Score::Value(10), Score::Value(10))
                    } else {
                        (Score::Value(9), Score::Value(9))
                    }
                }
                Rune::Mist => {
                    if is_ferric {
                        (
                            Score::Mod(ScoreModifier {
                                op: Op::Add(-1),
                                affected: AffectedCards::Row,
                            }),
                            Score::Mod(ScoreModifier {
                                op: Op::Add(-1),
                                affected: AffectedCards::Row,
                            }),
                        )
                    } else {
                        panic!("Non-Ferric Mist should not exist!")
                    }
                }
                Rune::Plague => (
                    Score::Mod(ScoreModifier {
                        op: Op::Mult(0),
                        affected: AffectedCards::Row,
                    }),
                    Score::Mod(ScoreModifier {
                        op: Op::Mult(0),
                        affected: AffectedCards::Row,
                    }),
                ),
                Rune::Weather => {
                    if is_ferric {
                        panic!("Ferric Weather should not exist!")
                    } else {
                        (
                            Score::Mod(ScoreModifier {
                                op: Op::Mult(2),
                                affected: AffectedCards::Row,
                            }),
                            Score::Mod(ScoreModifier {
                                op: Op::Mult(2),
                                affected: AffectedCards::Row,
                            }),
                        )
                    }
                }
                _ => panic!("Must provide a score string for {rune}"),
            }
        };
        Self {
            season,
            rune,
            court_score,
            garden_score,
        }
    }
}
