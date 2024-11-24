mod ability;
mod rune;
mod score;

use std::{
    fmt::Display,
    fs,
    io::{self, BufRead},
    path::Path,
};

use super::Season;
pub(crate) use ability::Ability;
pub(crate) use rune::Rune;
use score::ScoreModifier;
pub(crate) use score::{AffectedCards, Op, Score};

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
pub struct Card {
    season: Season,
    rune: Rune,
    court_score: Score,
    garden_score: Score,
}
impl Card {
    pub fn load_all(filename: &str) -> Vec<Self> {
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
    pub fn season(&self) -> Season {
        self.season
    }
    pub fn rune(&self) -> Rune {
        self.rune
    }
    pub fn court_score(&self) -> Score {
        self.court_score
    }
    pub fn garden_score(&self) -> Score {
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
