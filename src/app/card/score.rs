use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Score {
    Value(i32),
    Mod(RowScoreModifier),
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum RowScoreModifier {
    Mult(i32),
    Add(i32),
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Score::Value(v) => (*v).fmt(f),
            Score::Mod(sm) => sm.fmt(f),
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
            let modifier = match c {
                'x' => RowScoreModifier::Mult(val),
                '-' => RowScoreModifier::Add(-val),
                _ => panic!("Invalid score string: {}", value),
            };
            Score::Mod(modifier)
        } else {
            panic!("Invalid score string: {}", value);
        }
    }
}
impl Display for RowScoreModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            RowScoreModifier::Add(a) => format!("{a:+}"),
            RowScoreModifier::Mult(a) => format!("x{a}"),
        };
        format!("Row {}", op).fmt(f)
    }
}
