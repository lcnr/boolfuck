//! A rather slow [`Boolfuck`] interpreter, in case you want help me make this more efficient go to the corresponding github page.
//!
//! For tips/tutorials concerning [`Boolfuck`] visit the official homepage. It is also possible to translate [`Brainfuck`] in [`Boolfuck`] code with this crate.
//!
//! # Details
//!
//! * Input is taken from the console and the `end of line` character is removed,
//! because now programs written with `int getchar()`*( a function in `c`)* in mind work correctly.([`The Lost Kingdom`] for example)
//! This causes problems with other programs which rely on `null terminated` strings or the `end of line` character.
//!
//! * The commands `,` and `;` both work in little-endian order.
//!
//! * This crate is a **a lot** faster when run with `--release`!
//! [`Boolfuck`]:http://samuelhughes.com/boof/
//! [`Brainfuck`]:https://en.wikipedia.org/wiki/Brainfuck
//! [`The Lost Kingdom`]:http://web.archive.org/web/20111031121638/http://jonripley.com/i-fiction/games/LostKingdomBF.html


mod token;
mod program;

use token::Token;
pub use token::{ToString, ToToken};
use program::Program;

/// A simple struct which contains `boolfuck` code and can run that code.
///
/// # Examples
///
/// ```rust,no_run
/// use boolfuck::Boolfuck;
///
/// let program = Boolfuck::new(",>,>,>,>,>,>,>,<<<<<<<;>;>;>;>;>;>;>;");
///
/// program.run(false);
/// ```
pub struct Boolfuck {
    source: Vec<Token>,
}

impl Boolfuck {
    /// Creates a new instance of `Boolfuck` taking a `&str` as input, this string is used as the source code.
    ///
    /// # Panic
    ///
    /// In case there are unclosed brackets the program `panics`!
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use boolfuck::Boolfuck;
    ///
    /// let program = Boolfuck::new(",>,>,>,>,>,>,>,<<<<<<<;>;>;>;>;>;>;>;");
    ///
    /// program.run(false);
    /// ```
    pub fn new(source: &str) -> Self {
        Boolfuck {
            source: source.to_token(),
        }
    }

    /// Creates a new instance of `Boolfuck` taking a string of `Brainfuck` code as input, this string gets converted to `Boolfuck`.
    ///
    /// This is completely unoptimized!
    ///
    /// # Panic
    ///
    /// In case there are unclosed brackets the program `panics`!
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use boolfuck::Boolfuck;
    ///
    /// let program = Boolfuck::from_brainfuck(",.");
    ///
    /// program.run(false);
    /// ```
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

    /// Runs the code saved inside of the struct, in case present is true the value of all cells is shown after the program has finished.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use boolfuck::Boolfuck;
    ///
    /// let program = Boolfuck::new(",>,>,>,>,>,>,>,<<<<<<<;>;>;>;>;>;>;>;");
    ///
    /// program.run(true);
    /// ```
    pub fn run(&self, present: bool) {
        Program::new(&self.source).run(present);
    }

    /// Returns the representation of `Boolfuck` code used in this struct, this code can be converted to string using `to_string()`
    ///
    /// and then get used a a input for another `Boolfuck` instance.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use boolfuck::Boolfuck;
    /// use boolfuck::ToString;
    ///
    /// let program = Boolfuck::new(",>,>,>,>,>,>,>,<<<<<<<;>;>;>;>;>;>;>;");
    ///
    /// let program = Boolfuck::new(&program.get_tokens().to_string());
    ///
    /// program.run(true);
    /// ```
    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.source
    }

}

impl std::fmt::Debug for Boolfuck {
    /// Displays the used source code.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();

        for token in self.source.iter() {
            use Token::*;
            match *token {
                MoveLeft => output.push('<'),
                MoveRight => output.push('>'),
                BracketLeft => output.push('['),
                BracketRight => output.push(']'),
                Read => output.push(','),
                Write => output.push(';'),
                Flip => output.push('+'),
                EOF => (),
            }
        }

        write!(f, "Boolfuck: {{ {} }}", output)
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
