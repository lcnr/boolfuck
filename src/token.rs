#[derive(PartialEq,Copy,Clone)]
pub enum Token {
    MoveLeft,
    MoveRight,
    BracketLeft,
    BracketRight,
    Read,
    Write,
    Flip,
    EOF,
}

/// Takes `&self` and creates a new independent string.
pub trait ToString {
    fn to_string(&self) -> String;
}

/// Takes `&self` and creates a new independent `Vec<Token>`.
pub trait ToToken {
    fn to_token(&self) -> Vec<Token>;
}


impl ToString for Vec<Token> {
    fn to_string(&self) -> String {
        let mut    res = String::new();
        for item in self.iter() {
            use token::Token::*;
            match *item {
                MoveLeft => res.push('<'),
                MoveRight => res.push('>'),
                BracketLeft => res.push('['),
                BracketRight => res.push(']'),
                Read => res.push(','),
                Write => res.push(';'),
                Flip => res.push('+'),
                EOF => (),
            }
        }
        res
    }
}

impl ToToken for str {
    fn to_token(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut brackets: i32 = 0;

        for character in self.chars() {
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
        tokens.push(Token::EOF);


        if brackets != 0 {
            panic!("There are currently {} bracket(s) without a matching partner!",brackets);
        }
        else {
            tokens
        }
    }
}
