use strum_macros::Display;

#[derive(Clone, Copy, Display, PartialEq, Debug)]
pub(crate) enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
    Ferric,
}
impl From<&str> for Season {
    fn from(value: &str) -> Self {
        match value {
            "Spring" => Season::Spring,
            "Summer" => Season::Summer,
            "Autumn" => Season::Autumn,
            "Winter" => Season::Winter,
            "Ferric" => Season::Ferric,
            _ => panic!("Invalid season {}", value),
        }
    }
}
