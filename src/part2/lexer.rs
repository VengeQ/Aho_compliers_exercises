use std::collections::HashMap;
use std::time::Duration;
use std::ops::Deref;
use core::fmt;
use std::fmt::Debug;

trait Token {
    fn value(&self) -> String;
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[derive(Default, Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Clone)]
struct Word {
    word: String
}

impl Word {
    pub fn new(word: String) -> Self { Word { word } }
}

impl Token for Word {
    fn value(&self) -> String {
        self.word.to_owned()
    }
}


#[derive(Default, Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Clone)]
struct Id {
    id: String
}

impl Id {
    pub fn new(id: String) -> Self { Id { id } }
}

impl Token for Id {
    fn value(&self) -> String {
        self.id.to_owned()
    }
}

#[derive(Default, Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
struct Num {
    num: i32
}

impl Num {
    pub fn new(value: i32) -> Self { Num { num: value } }
}

impl<'t> Token for Num {
    fn value(&self) -> String {
        (self.num.to_string())
    }
}


type words = HashMap<Word, String>;
type ids = HashMap<Id, String>;
struct Lexer {
    index: usize,
    line: i32,
    peek: char,
    words: Box<words>,
    ids:Box<ids>
}


impl Lexer {
    pub fn new() -> Self {
        let mut lexer = Lexer {
            index: 0,
            line: 1,
            peek: ' ',
            words: Default::default(),
            ids:Default::default()
        };
        lexer.words.insert(Word::new("TRUE".to_owned()), "true".to_owned());
        lexer.words.insert(Word::new("FALSE".to_owned()), "false".to_owned());
        lexer
    }

    pub fn scan_string(&mut self, input: &str) -> Box<dyn Token> {
        let mut for_scan = input.split_at(self.index).1.chars();
        loop {
            self.index += 1;
            self.peek = for_scan.next().unwrap_or_else(|| '\0');
            match self.peek {
                ' ' | '\t' => {}
                '\n' => self.line += 1,
                '\0' => {
                    self.line = -1;
                    return Box::new(Word::new("EOF".to_string()))
                }
                _ => break
            }
        }

        if char::is_digit(self.peek, 10) {
            let mut result = 0;
            loop {
                self.index += 1;
                result = 10 * result + char::to_digit(self.peek, 10).unwrap();
                self.peek = for_scan.next().unwrap_or_else(|| '\0');
                if !char::is_digit(self.peek, 10) { return Box::new(Num::new(result as i32)); }
            }
        }

        if char::is_alphabetic(self.peek){
            let mut result ="".to_owned();
            loop{
                self.index += 1;
                result.push(self.peek);
                self.peek = for_scan.next().unwrap_or_else(|| '\0');
                if !char::is_alphabetic(self.peek){
                    let word = Word::new(result.clone());
                    let id = Id::new(result.to_owned());
                    if self.words.get(&word).is_none() && self.ids.get(&id).is_none(){
                        self.ids.insert(id.clone(),id.id.clone());
                        return Box::new(Word::new(result.to_string()))
                    }
                }
            }
        }

        return Box::new(Word::new("EOF".to_string()))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_lexer_test() {
        use super::*;
        let l = Lexer::new();
        assert_eq!(l.peek, ' ');
        assert_eq!(l.line, 1);
        let tr = Word::new("TRUE".to_owned());
        assert_eq!(l.words.get(&tr).unwrap().to_owned(), "true".to_string());
    }

    #[test]
    fn scan_string_test() {
        use super::*;
        let mut l = Lexer::new();
        let cur = l.line;
        while l.line != -1  {
            println!("get token: {:?}", l.scan_string("123 234  hello  ").deref());

        }
    }
}