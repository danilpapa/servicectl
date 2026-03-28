use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(PartialEq, EnumIter)]
pub enum ActionChoice {
    BuildSelected = 0,
    BuildAll = 1,
}

impl ActionChoice {
    pub fn toggle(&mut self) {
        if self == &ActionChoice::BuildSelected {
            *self = ActionChoice::BuildAll
        } else {
            *self = ActionChoice::BuildSelected
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            ActionChoice::BuildSelected => 0,
            ActionChoice::BuildAll => 1,
        }
    }
}

impl Display for ActionChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionChoice::BuildSelected => write!(f, "Build only selected services"),
            ActionChoice::BuildAll => write!(f, "Build all services"),
        }
    }
}