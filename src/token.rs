use std::fmt;

/// An enum with all possible `boolfuck` commands and an `end of file` token.
#[derive(PartialEq, Copy, Clone)]
pub enum Token {
    MoveLeft,
    MoveRight,
    BracketLeft,
    BracketRight,
    Read,
    Write,
    Flip,
}


impl fmt::Debug for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Token {{ {} }}", self)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Token::*;
        write!(fmt, "{}", match self {
            MoveLeft => '>',
            MoveRight => '<',
            BracketLeft => '[',
            BracketRight => ']',
            Read => ',',
            Write => ';',
            Flip => '+'
        })
    }
}
