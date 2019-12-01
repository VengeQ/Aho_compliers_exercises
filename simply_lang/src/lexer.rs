use std::cell::Cell;
use std::fmt::{Debug, Formatter, Error};

const BUFFERSIZE: usize = 2048;

trait Token {
    fn tag(&self) -> (Tag, Option<String>);
}

impl Debug for dyn Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let tag = self.tag();
        if let Some(attr) = tag.1 {
            write!(f, "It's [{:?}] token with attribute [{:?}].", tag.0, attr)
        } else {
            write!(f, "It's [{:?}] token without attribute.", tag.0)
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Num {
    value: u64,
}

impl Num {}


impl Token for Num {
    fn tag(&self) -> (Tag, Option<String>) {
        (Tag::NUMBER, Option::Some(self.value.to_string()))
    }
}

pub struct Word {}

impl Word {}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Tag {
    IF,
    THEN,
    ELSE,
    FOR,
    AND,
    EQ,
    NE,
    MINUS,
    ID,
    NUMBER,
}


pub struct Lexer {
    buffer: Vec<char>,
    lexeme_begin: Cell<usize>,
    forward: Cell<usize>,
}


impl Lexer {
    pub fn new(input: String) -> Self {
        let mut chars: Vec<char> = input.chars().take(2048).collect();
        chars.push('\0');
        Self {
            buffer: chars,
            lexeme_begin: Cell::new(0),
            forward: Cell::new(0),
        }
    }

    fn next_token() -> Box<dyn Token> {
        unimplemented!()
    }

    fn scan(&self) -> Option<Box<Token>> {
        let lexeme_begin = &self.lexeme_begin;
        let forward = &self.forward;
        //skip whitespaces
        while self.buffer[forward.get()].is_whitespace() {
            dbg!("Whitespace at position: {}", forward.get());
            forward.set(forward.get() + 1);
        }
        lexeme_begin.set(forward.get());

        if self.buffer[forward.get()].is_numeric() {
            forward.set(forward.get() + 1);
            while self.buffer[forward.get()].is_numeric() {
                forward.set(forward.get() + 1);
            }

            let number = self.buffer[lexeme_begin.get()..forward.get()].iter()
                .fold("".to_string(), |mut acc, value| {
                    acc.push(*value);
                    acc
                }).parse::<u64>().unwrap();

            let token: Num = Num {
                value: number.to_owned()
            };

            dbg!("Get token [{:?}] in position [{:?}]", &token ,lexeme_begin.get());

            return Some(Box::new(token));
        }
        return None;
    }

    fn test_print(&self) {
        self.buffer.iter().for_each(|x| println!("{}", x));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let l = Lexer::new("hello".to_owned());
    }

    #[test]
    fn test_print_test() {
        let l = Lexer::new("  hello".to_owned());
        l.test_print()
    }

    #[test]
    fn scan_test() {
        let l = Lexer::new(" 123  422".to_owned());
        while let Some(token) = l.scan() {
            println!("{:?}", token);
        }
    }
}