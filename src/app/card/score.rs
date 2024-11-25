use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum Score {
    Value(i32),
    Mod(RowScoreModifier),
}
impl Score {
    pub(crate) fn is_value(&self) -> bool {
        match self {
            Score::Value(_) => true,
            _ => false,
        }
    }
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
impl Display for RowScoreModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            RowScoreModifier::Add(a) => format!("{a:+}"),
            RowScoreModifier::Mult(a) => format!("x{a}"),
        };
        format!("Row {}", op).fmt(f)
    }
}
