#[derive(Debug, Clone, Copy, Default)]
pub enum WolframCodeState {
    #[default]
    Dead,
    Alive,
}

impl Into<bool> for WolframCodeState {
    fn into(self) -> bool {
        match self {
            Self::Alive => true,
            Self::Dead => false,
        }
    }
}

impl From<bool> for WolframCodeState {
    fn from(value: bool) -> Self {
        if value {
            Self::Alive
        } else {
            Self::Dead
        }
    }
}
