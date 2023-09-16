use std::fmt;

#[derive(Debug)]
pub enum Group {
    Number(usize),
    Name(String),
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Group::Number(n) => write!(f, "{}", n),
            Group::Name(n) => write!(f, "{}", n),
        }
    }
}
impl From<usize> for Group {
    fn from(n: usize) -> Self {
        Group::Number(n)
    }
}
impl From<&str> for Group {
    fn from(n: &str) -> Self {
        Self::Name(n.to_owned())
    }
}
impl From<String> for Group {
    fn from(n: String) -> Self {
        Self::Name(n)
    }
}
