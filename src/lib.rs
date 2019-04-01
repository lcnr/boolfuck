//! A simple [`Boolfuck`] interpreter. The code is available at [github].
//!
//! For tips/tutorials concerning [`Boolfuck`] visit the official homepage. It is also possible to translate [`Brainfuck`] in [`Source`] code with this crate.
//!
//! # Details
//!
//! * Input is taken from the console and the `end of line` character is removed,
//! because now programs written with `int getchar()`*( a function in `c`)* in mind work correctly.([`The Lost Kingdom`] for example)
//! This causes problems with other programs which rely on `null terminated` strings or the `end of line` character.
//!
//! * The commands `,` and `;` both work in little-endian order.
//! [`Boolfuck`]:http://samuelhughes.com/boof/
//! [`Brainfuck`]:https://en.wikipedia.org/wiki/Brainfuck
//! [`The Lost Kingdom`]:http://web.archive.org/web/20111031121638/http://jonripley.com/i-fiction/games/LostKingdomBF.html
//! [github]:https://github.com/Nijaitchy/boolfuck/


mod token;
mod program;

pub use token::Token as Token;
use program::Program;

/// A simple struct which contains compilable `boolfuck` code.
pub struct Source {
    source: Vec<Token>,
}

impl Source {
    /// Creates a new instance of `Source` from `source`.
    ///
    /// # Panic
    ///
    /// In case there are unclosed brackets the program `panics`!
    pub fn new(source: &str) -> Self {
        let mut tokens = Vec::new();
        let mut brackets: i32 = 0;

        for character in source.chars() {
            match character {
                '<' => tokens.push(Token::MoveLeft),
                '>' => tokens.push(Token::MoveRight),
                '[' => { brackets += 1; tokens.push(Token::BracketLeft); },
                ']' => {
                    if brackets > 0 {
                        brackets -= 1;
                        tokens.push(Token::BracketRight);
                    }
                    else {
                        panic!("Found a right bracket without a preceding left one!");
                    }
                },
                ',' => tokens.push(Token::Read),
                ';' => tokens.push(Token::Write),
                '+' => tokens.push(Token::Flip),
                _ => (),
            }
        }


        if brackets != 0 {
            panic!("There are currently {} bracket(s) without a matching partner!",brackets);
        }
        else {
            Source {
                source: tokens,
            }
        }
    }

    /// Creates a new instance of `Source` taking a string of `Brainfuck` code as input, this string gets converted to `Source`.
    ///
    /// This is completely unoptimized!
    ///
    /// # Panic
    ///
    /// In case there are unclosed brackets the program `panics`!
    pub fn from_brainfuck(source: &str) -> Self {
        let mut boolfuck_source = String::new();

        for character in source.chars() {
            match character {
                '+' => boolfuck_source.push_str(">[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<"),
                '-' => boolfuck_source.push_str(">>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<"),
                '<' => boolfuck_source.push_str("<<<<<<<<<"),
                '>' => boolfuck_source.push_str(">>>>>>>>>"),
                ',' => boolfuck_source.push_str(">,>,>,>,>,>,>,>,<<<<<<<<"),
                '.' => boolfuck_source.push_str(">;>;>;>;>;>;>;>;<<<<<<<<"),
                '[' => boolfuck_source.push_str(">>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]"),
                ']' => boolfuck_source.push_str(">>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]"),
                _ => (),
            }
        }

        Self::new(&boolfuck_source)
    }

    /// Creates a program from `self`.
    pub fn gen(self) -> Program {
        Program::new(self.source)
    }

    pub fn len(&self) -> usize {
        self.source.len()
    }

    /// removes all simple duplicates
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # use boolfuck::Source;
    /// let mut source = Source::new("[]++ >++< <>");
    /// source.dedup();
    /// assert_eq!(source.len(), 0);
    /// 
    /// let mut source = Source::new("+[]++[+,;++++>>>>><<<][]");
    /// source.dedup();
    /// assert_eq!(source.len(), 3);
    /// 
    /// ```
    pub fn dedup(&mut self) {
        let mut iter = std::mem::replace(&mut self.source, Vec::new()).into_iter();

        let mut prev_tok = None;
        while let Some(tok) = iter.next() {
            match (prev_tok, tok) {
                (Some(Token::Flip), Token::Flip) |
                (Some(Token::MoveLeft), Token::MoveRight) |
                (Some(Token::MoveRight), Token::MoveLeft) => prev_tok = self.source.pop(),
                (Some(Token::BracketRight), Token::BracketLeft) |
                (None, Token::BracketLeft) => {
                    Self::skip_to_matching(&mut iter);
                }
                (prev, tok) => {
                    if let Some(prev) = prev {
                        self.source.push(prev);
                    }
                    prev_tok = Some(tok);
                }
            }
        }
        
        if let Some(prev) = prev_tok {
            self.source.push(prev);
        }
    }

    fn skip_to_matching(iter: &mut impl Iterator<Item=Token>) {
        let mut brackets = 1;
        while brackets != 0 {
            match iter.next() {
                Some(Token::BracketLeft) => brackets += 1,
                Some(Token::BracketRight) => brackets -= 1,
                Some(_) => (),
                None => unreachable!(),
            }
        }
    }
}

impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::with_capacity(self.source.len());

        for token in self.source.iter() {
            use std::fmt::Write;
            write!(output, "{}", token)?
        }

        write!(f, "Source: {{ \"{}\" }}", output)
    }
}