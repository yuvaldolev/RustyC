use std::fmt;

#[derive(Clone, Debug)]
pub enum TokenCategory {
    Primary,
}

impl fmt::Display for TokenCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primary => write!(f, "primary"),
        }
    }
}
