
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameOfLifeState {
    #[default]
    Dead,
    Alive,
}

impl Into<bool> for GameOfLifeState {
    fn into(self) -> bool {
        match self {
            Self::Alive => true,
            Self::Dead => false,
        }
    }
}

impl From<bool> for GameOfLifeState {
    fn from(value: bool) -> Self {
        if value {
            Self::Alive
        } else {
            Self::Dead
        }
    }
}
