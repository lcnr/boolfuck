use token::Token;
use std::collections::VecDeque;
use Token::*;



pub struct Program<'a> {
    source: &'a Vec<Token>,
    cells: VecDeque<u8>,
    input: VecDeque<u8>,

    write: u8,

    source_ptr: usize,
    cells_ptr: usize,
    read_ptr: u8,
    write_ptr: u8,
}

impl<'a> Program<'a> {
    pub fn new(source: &'a Vec<Token>) -> Self {
        let mut cells = VecDeque::new();
        cells.push_back(0b00_00_00_00);

        let input = VecDeque::new();
        let write: u8 = 0b00_00_00_00;

        let source_ptr = 0;
        let cells_ptr = 0;
        let read_ptr = 0b10_00_00_00;
        let write_ptr = 0b00_00_00_01;

        Program {
            source,
            cells,
            input,

            write,

            source_ptr,
            cells_ptr,
            read_ptr,
            write_ptr,
        }
    }

    pub fn run(&mut self, present: bool) {
        'running: loop {
            match self.source[self.source_ptr] {
                MoveLeft => self.move_left(),
                MoveRight => self.move_right(),
                BracketLeft => self.bracket_left(),
                BracketRight => self.bracket_right(),
                Read => self.read(),
                Write => self.write(),
                Flip => self.flip(),
                EOF => break 'running,
            }
        }

        if present {
            println!("\n\nThese are all cells after the program has finished:");

            let mut counter = 0;
            for item in self.cells.iter().rev() {
                let mut cell = 0b00_00_00_01;
                'cell_iter: loop {
                    if counter % 8 == 0 {
                        print!("\n");
                    }
                    else if counter % 4 == 0 {
                        print!("- ");
                    }
                    counter += 1;

                    if item & cell != 0 {
                        print!("1 ");
                    }
                    else {
                        print!("0 ");
                    }
                    if cell == 0b10_00_00_00 {
                        break 'cell_iter;
                    }
                    else {
                        cell = cell << 1;
                    }
                }
            }
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
        if self.read_cell() {
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
        let mut open_brackets: u8 = 1;

        while open_brackets != 0 {
            self.source_ptr -= 1;

            use Token::*;
            match self.source[self.source_ptr] {
                BracketLeft => open_brackets -= 1,
                BracketRight => open_brackets += 1,
                _ => (),
            }
        }
        //self.source_ptr += 1;
    }

    fn read(&mut self) {
        self.source_ptr += 1;

        if self.read_ptr & 0b10_00_00_00 != 0 {
            self.input.pop_front();

            while self.input.is_empty() {
                use std::io::Read;
                self.input.push_back(::std::io::stdin().bytes().next().and_then(|result| result.ok()).unwrap());
                if *self.input.back().unwrap() == 10 {
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
            self.one_cell();
        }
        else {
            self.zero_cell();
        }
    }

    fn write(&mut self) {
        self.source_ptr += 1;

        if self.read_cell() {
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



    fn read_cell(&self) -> bool {
        let item = 0b00_00_00_01 << self.cells_ptr % 8;

        self.cells[self.cells_ptr/8] & item != 0
    }

    fn flip_cell(&mut self) {
        let item = 0b00_00_00_01 << self.cells_ptr % 8;

        self.cells[self.cells_ptr/8] ^= item;
    }

    fn one_cell(&mut self) {
        let item = 0b00_00_00_01 << self.cells_ptr % 8;

        self.cells[self.cells_ptr/8] |= item;
    }

    fn zero_cell(&mut self) {
        let item = 0b00_00_00_01 << self.cells_ptr % 8;

        self.cells[self.cells_ptr/8] &= !item;
    }
}
