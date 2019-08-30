use std::collections::HashMap;
use std::ops::Deref;
use core::fmt;
use std::fmt::Debug;
use std::str::Chars;

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Clone)]
enum Tag {
    NUM,
    ID,
    FALSE,
    TRUE,
    OP,
    EOF
}

impl PartialEq for dyn Token {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

trait Token {
    fn tag(&self) -> &Tag;
    fn value(&self) -> String;
}

impl Debug for dyn Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

//Использую отдельно ключевые слова
#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Clone)]
struct Word {
    tag: Tag,
    word: String,
}

impl Word {
    pub fn new(tag: Tag, word: String) -> Self { Word { tag, word } }
}

impl Token for Word {
    fn tag(&self) -> &Tag {
        &self.tag
    }

    fn value(&self) -> String {
        self.word.to_owned()
    }
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
struct Num {
    tag: Tag,
    num: i32,
}

impl Num {
    pub fn new(value: i32) -> Self { Num { tag: Tag::NUM, num: value } }
}

impl Token for Num {
    fn tag(&self) -> &Tag {
        &self.tag
    }
    fn value(&self) -> String {
        (self.num.to_string())
    }
}


type Words = HashMap<String, Word>;
type Lines = HashMap<usize, Box<Vec<Box<dyn Token>>>>;

struct Lexer {
    index: usize,
    line: i32,
    peek: char,
    words: Box<Words>,
    lines: HashMap<usize, Box<Vec<Box<dyn Token>>>>,
}

// ***********************************************
// _____________ЛЕКСИЧЕСКИЙ АНАЛИЗАТОР____________
// ***********************************************
impl Lexer {
    pub fn new() -> Self {
        let mut lexer = Lexer {
            index: 0,
            line: 1,
            peek: ' ',
            words: Default::default(),
            lines: Default::default(),
        };
        lexer.words.insert("TRUE".to_owned(), Word::new(Tag::TRUE, "TRUE".to_owned()));
        lexer.words.insert("FALSE".to_owned(), Word::new(Tag::FALSE, "FALSE".to_owned()));
        lexer.words.insert("EOF".to_owned(), Word::new(Tag::EOF, "EOF".to_owned()));
        lexer.lines.insert(1_usize, Box::new(Vec::new()));
        lexer
    }

    fn reserve(&mut self, word: Word) {
        self.words.insert(word.value(), word);
    }

    fn read_char_or_return_eof(&mut self, input: &mut Chars) -> char {
        if self.peek != '\0' {
            self.peek = input.next().unwrap_or('\0');
            if self.peek != '\0' {
                self.index += 1;
            }
        }
        self.peek
    }

    fn return_ptr(&mut self, input: &mut Chars) {
        self.peek = input.next_back().unwrap_or('\0');
        self.index -= 1;
    }

    pub fn full_scan(&mut self, input: &str) -> &Lines {
        while self.line != -1 {
            let a = self.scan(input);
            if &a.value()[..] == "EOF" {
                break;
            }
        }
        &self.lines
    }

    pub fn scan(&mut self, input: &str) -> Box<dyn Token> {
        let mut for_scan = input.split_at(self.index).1.chars();
        //убрать пробелы и табуляции
        loop {
            self.read_char_or_return_eof(&mut for_scan);
            match self.peek {
                ' ' | '\t' => {}
                '\n' => {
                    self.line += 1;
                    self.lines.insert(self.line as usize, Box::new(Vec::new()));
                }
                '\0' => {
                    self.line = -1;
                    self.lines.entry(self.line as usize).and_modify(|x|
                        x.push(Box::new(Word::new(Tag::EOF, "EOF".to_string()))));
                    return Box::new(Word::new(Tag::EOF, "EOF".to_string()));
                }
                _ => break
            }
        }
        //Комментарии
        if self.peek == '/' {
            self.read_char_or_return_eof(&mut for_scan);
            if self.peek == '/' {
                loop {
                    self.read_char_or_return_eof(&mut for_scan);
                    if self.peek == '\0' {
                        break;
                    }
                    if self.peek == '\n' {
                        self.line += 1;
                        self.lines.insert(self.line as usize, Box::new(Vec::new()));
                        self.read_char_or_return_eof(&mut for_scan);
                        break;
                    }
                }
            }
            if self.peek == '*' {
                loop {
                    self.read_char_or_return_eof(&mut for_scan);
                    if self.peek == '\0' {
                        panic!("Unclosed comment");
                    }
                    if self.peek == '*' {
                        self.read_char_or_return_eof(&mut for_scan);
                        match self.peek {
                            '\0' => panic!("Unclosed comment"),
                            '\\' => break,
                            _ => {}
                        }
                    }
                }
            }
        }


        match self.peek{
            '+' =>{
                let word = Word::new(Tag::OP, "+".to_owned());
                self.lines.entry(self.line as usize).and_modify(|x|
                    x.push(Box::new(word.clone())));
                return Box::new(word);
            }
            '-' =>{
                let word = Word::new(Tag::OP, "-".to_owned());
                self.lines.entry(self.line as usize).and_modify(|x|
                    x.push(Box::new(word.clone())));
                return Box::new(word);
            }
            _ => {}
        }

        //Знаки сравнения
        match self.peek {
            '<' => {
                self.read_char_or_return_eof(&mut for_scan);
                if self.peek == '=' {
                    let word = Word::new(Tag::OP, "<=".to_owned());
                    self.lines.entry(self.line as usize).and_modify(|x|
                        x.push(Box::new(word.clone())));
                    return Box::new(word);
                } else {
                    self.return_ptr(&mut for_scan);
                    let word = Word::new(Tag::OP, "<".to_owned());
                    self.lines.entry(self.line as usize).and_modify(|x|
                        x.push(Box::new(word.clone())));
                    return Box::new(word);
                }
            }
            '!' => {
                self.read_char_or_return_eof(&mut for_scan);
                if self.peek == '=' {
                    self.return_ptr(&mut for_scan);
                    let word = Word::new(Tag::OP, "!=".to_owned());
                    self.lines.entry(self.line as usize).and_modify(|x|
                        x.push(Box::new(word.clone())));
                    return Box::new(word);
                }
            }
            '>' => {
                self.read_char_or_return_eof(&mut for_scan);
                if self.peek == '=' {
                    let word = Word::new(Tag::OP, ">=".to_owned());
                    self.lines.entry(self.line as usize).and_modify(|x|
                        x.push(Box::new(word.clone())));
                    return Box::new(word);
                } else {
                    self.return_ptr(&mut for_scan);
                    let word = Word::new(Tag::OP, ">".to_owned());
                    self.lines.entry(self.line as usize).and_modify(|x|
                        x.push(Box::new(word.clone())));
                    return Box::new(word);
                }
            }
            '=' => {
                self.read_char_or_return_eof(&mut for_scan);
                if self.peek == '=' {
                    let word = Word::new(Tag::OP, "==".to_owned());
                    self.lines.entry(self.line as usize).and_modify(|x|
                        x.push(Box::new(word.clone())));
                    return Box::new(word);
                } else {
                    self.return_ptr(&mut for_scan);
                    let word = Word::new(Tag::OP, "=".to_owned());
                    self.lines.entry(self.line as usize).and_modify(|x|
                        x.push(Box::new(word.clone())));
                    return Box::new(word);
                }
            }
            _ => {}
        }

        //проверка на число
        if char::is_digit(self.peek, 10) {
            let mut result = 0;
            loop {
                result = 10 * result + char::to_digit(self.peek, 10).unwrap();
                self.read_char_or_return_eof(&mut for_scan);
                if !char::is_digit(self.peek, 10) {
                    self.return_ptr(&mut for_scan);
                    self.lines.entry(self.line as usize).and_modify(|x|
                        x.push(Box::new(Num::new(result as i32))));
                    return Box::new(Num::new(result as i32));
                }
            }
        }
        //проверка на тэг
        if char::is_alphabetic(self.peek) {
            let mut result = "".to_owned();
            loop {
                result.push(self.peek);
                self.read_char_or_return_eof(&mut for_scan);
                if !char::is_alphabetic(self.peek) {
                    self.return_ptr(&mut for_scan);
                    let word = Word::new(Tag::ID, result.clone());
                    if self.words.get(&result).is_none() {
                        self.words.insert(result.clone(), word.clone());
                    }
                    self.lines.entry(self.line as usize).and_modify(|x|
                        x.push(Box::new(word.clone())));
                    return Box::new(word);
                }
            }
        }
        return Box::new(Word::new(Tag::EOF, "EOF".to_string()))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_lexer_test() {
        use super::*;
        let mut l = Lexer::new();
        assert_eq!(l.peek, ' ');
        assert_eq!(l.line, 1);
        let tr = Word::new(Tag::ID, "id".to_owned());
        l.reserve(tr);
        assert_eq!(l.words.get("TRUE").unwrap().to_owned().tag, Tag::TRUE);
        assert_eq!(l.words.get("id").unwrap().to_owned().tag, Tag::ID);
    }

    #[test]
    fn scan_string_test() {
        use super::*;
        let mut l = Lexer::new();

        let input = String::from("a = 3\n") + "b =23\n" + "a + b\n" + "a >= 4\n" + "c!=a";
        while l.line != -1 {
            let a = l.scan(&input);
            println!("get token: {:?}", a.deref());
            if &a.value()[..] == "EOF" {
                break;
            }
        }
        l.words.iter().for_each(|x|println!("w: {:?}", x));
        let w1: Box<dyn Token> = Box::new(Word::new(Tag::ID, "b".to_owned()));
        let w2: Box<dyn Token> = Box::new(Word::new(Tag::ID, "=".to_owned()));
        let w3: Box<dyn Token> = Box::new(Word::new(Tag::ID, "23".to_owned()));
        assert_eq!(l.lines.get(&2_usize).unwrap(),
                   &Box::new(vec![w1, w2, w3]));
        l.lines.iter().for_each(|x| println!("{:?}", x));
    }

    #[test]
    #[should_panic(expected = "Unclosed comment")]
    fn scan_string_panic_test() {
        use super::*;
        let mut l = Lexer::new();
        while l.line != -1 {
            let a = l.scan("123 234  hello\n now be //    asdasdfasd\nas\n let 23 /*vasya");
            ;
            if &a.value()[..] == "EOF" {
                break;
            }
        }
    }
}