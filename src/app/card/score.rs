use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Score {
    Value(i32),
    Mod(ScoreModifier),
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Op {
    Mult(i32),
    Add(i32),
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum AffectedCards {
    Adjacent,
    Row,
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct ScoreModifier {
    pub op: Op,
    pub affected: AffectedCards,
}

impl Score {
    pub fn from_string(string: &str) -> Self {
        let first_char = string.chars().nth(0).expect("Empty score string");
        if "0123456789".contains(first_char) {
            let val: i32 = string.parse().expect("Invalid score string");
            Score::Value(val)
        } else if first_char == 'R' {
            assert!(string.len() >= 3, "Invalid score string: {}", string);
            let c = string.chars().nth(1).unwrap();
            let s: String = string.chars().skip(2).collect();
            let val: i32 = s.parse().expect("Invalid score string");
            let op = match c {
                'x' => Op::Mult(val),
                '-' => Op::Add(-val),
                _ => panic!("Invalid score string: {}", string),
            };
            Score::Mod(ScoreModifier {
                op,
                affected: AffectedCards::Row,
            })
        } else {
            panic!("Invalid score string: {}", string);
        }
    }
    pub(crate) fn is_value(&self) -> bool {
        match self {
            Score::Mod(_) => false,
            Score::Value(_) => true,
        }
    }
}
impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Score::Value(v) => (*v).fmt(f),
            Score::Mod(sm) => {
                let aff = match sm.affected {
                    AffectedCards::Adjacent => String::from("adj"),
                    AffectedCards::Row => String::from("row"),
                };
                let op = match sm.op {
                    Op::Add(a) => {
                        if a < 0 {
                            format!("{a}")
                        } else {
                            format!("+{a}")
                        }
                    }
                    Op::Mult(a) => format!("x{a}"),
                };
                format!("{aff} {op}").fmt(f)
            }
        }
    }
}
impl From<&str> for Score {
    fn from(value: &str) -> Self {
        let first_char = value.chars().nth(0).expect("Empty score string");
        if "0123456789".contains(first_char) {
            let val: i32 = value.parse().expect("Invalid score string");
            Score::Value(val)
        } else if first_char == 'R' {
            assert!(value.len() >= 3, "Invalid score string: {}", value);
            let c = value.chars().nth(1).unwrap();
            let s: String = value.chars().skip(2).collect();
            let val: i32 = s.parse().expect("Invalid score string");
            let op = match c {
                'x' => Op::Mult(val),
                '-' => Op::Add(-val),
                _ => panic!("Invalid score string: {}", value),
            };
            Score::Mod(ScoreModifier {
                op,
                affected: AffectedCards::Row,
            })
        } else {
            panic!("Invalid score string: {}", value);
        }
    }
}
impl Display for ScoreModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let aff = match self.affected {
            AffectedCards::Adjacent => String::from("Adj"),
            AffectedCards::Row => String::from("Row"),
        };
        let op = match self.op {
            Op::Add(a) => format!("{a:+}"),
            Op::Mult(a) => format!("x{a}"),
        };
        format!("{} {}", aff, op).fmt(f)
    }
}
