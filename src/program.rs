use token::Token;
use std::collections::VecDeque;

pub struct Program {
    source: Vec<Token>,
    cells: VecDeque<u8>,
    input: VecDeque<u8>,
    stack: Vec<usize>,

    write: u8,

    source_ptr: usize,
    cells_ptr: usize,
    read_ptr: u8,
    write_ptr: u8,
}

impl Program {
    pub(crate) fn new(source: Vec<Token>) -> Self {
        let mut cells = VecDeque::new();
        cells.push_back(0b00_00_00_00);

        Program {
            source,
            cells,
            input: VecDeque::new(),
            stack: Vec::new(),

            write: 0,

            source_ptr: 0,
            cells_ptr: 0,
            read_ptr: 0b10_00_00_00,
            write_ptr: 0b00_00_00_01
        }
    }

    pub fn tape(&self) -> (&[u8], &[u8]) {
        self.cells.as_slices()
    }

    pub fn run(&mut self) {
        while let Some(tok) = self.source.get(self.source_ptr).cloned() {
            match tok {
                Token::MoveLeft => self.move_left(),
                Token::MoveRight => self.move_right(),
                Token::BracketLeft => self.bracket_left(),
                Token::BracketRight => self.bracket_right(),
                Token::Read => self.read(),
                Token::Write => self.write(),
                Token::Flip => self.flip(),
            }
        }

        if self.write_ptr != 0b00_00_00_01 {
            print!("{}", char::from(self.write));
        }
    }


    fn move_left(&mut self) {
        self.source_ptr += 1;

        if self.cells_ptr == 0 {
            self.cells_ptr = 7;
            self.cells.push_front(0b00_00_00_00);
        }
        else {
            self.cells_ptr -= 1;
        }
    }

    fn move_right(&mut self) {
        self.source_ptr += 1;

        self.cells_ptr += 1;
        if !(self.cells_ptr / 8 < self.cells.len()) {
            self.cells.push_back(0b00_00_00_00);
        }
    }

    fn bracket_left(&mut self) {
        if self.get_cell() {
            self.stack.push(self.source_ptr);
            self.source_ptr += 1;
        }
        else {
            let mut open_brackets: u8 = 1;

            while open_brackets != 0 {
                self.source_ptr += 1;

                use Token::*;
                match self.source[self.source_ptr] {
                    BracketLeft => open_brackets += 1,
                    BracketRight => open_brackets -= 1,
                    _ => (),
                }
            }
            self.source_ptr += 1;
        }
    }

    fn bracket_right(&mut self) {
        self.source_ptr = self.stack.pop().unwrap();
    }

    fn read(&mut self) {
        self.source_ptr += 1;

        if self.read_ptr & 0b10_00_00_00 != 0 {
            self.input.pop_front();

            while self.input.is_empty() {
                use std::io::Read;
                self.input.push_back(::std::io::stdin().bytes().next().and_then(|result| result.ok()).unwrap());
                if *self.input.back().unwrap() == '\n' as u8 {
                    self.input.pop_back();
                }
            }
            self.read_ptr = 0b00_00_00_01;
        }
        else {
            self.read_ptr = self.read_ptr << 1;
        }


        let input = self.input.front().expect("Error inside of the 'read' function").clone();
        if self.read_ptr & input != 0 {
            self.set_cell(true);
        }
        else {
            self.set_cell(false);
        }
    }

    fn write(&mut self) {
        self.source_ptr += 1;

        if self.get_cell() {
            self.write |= self.write_ptr;
        }

        if self.write_ptr & 0b10_00_00_00 != 0 {
            print!("{}", char::from(self.write));

            self.write_ptr = 0b00_00_00_01;
            self.write = 0b00_00_00_00;
        }
        else {
            self.write_ptr = self.write_ptr << 1;
        }
    }

    fn flip(&mut self) {
        self.source_ptr += 1;

        self.flip_cell();
    }



   
    #[inline]
    fn flip_cell(&mut self) {
        let v = !self.get_cell();
        self.set_cell(v);
    }

    #[inline]
    fn set_cell(&mut self, v: bool) {
        let item = 0b00_00_00_01 << self.cells_ptr % 8;
        if v {
            self.cells[self.cells_ptr/8] |= item;
        }
        else {
            self.cells[self.cells_ptr/8] &= !item;
        }
    }

     fn get_cell(&self) -> bool {
        let item = 0b00_00_00_01 << self.cells_ptr % 8;

        self.cells[self.cells_ptr/8] & item != 0
    }
}
