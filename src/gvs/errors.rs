#[derive(Clone)]
pub struct ParseError {
    pub msg: String,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, ":9 {}", self.msg)
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, ":( {}", self.msg)
    }
}

pub struct ExtendedUnsvgError {
    pub msg: String,
}
impl std::error::Error for ExtendedUnsvgError {}

impl std::fmt::Debug for ExtendedUnsvgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, ":9 {}", self.msg)
    }
}

impl std::fmt::Display for ExtendedUnsvgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, ":( {}", self.msg)
    }
}
