use std::collections::HashMap;
use std::ops::Deref;
use core::fmt;
use std::fmt::Debug;
use std::str::Chars;

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Clone)]
enum Tag {
    NUM,
    WORD,
    FALSE,
    TRUE,
    OP,
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
                        x.push(Box::new(Word::new(Tag::WORD, "EOF".to_string()))));
                    return Box::new(Word::new(Tag::WORD, "EOF".to_string()));
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
                    let word = Word::new(Tag::OP, "<".to_owned());
                    self.lines.entry(self.line as usize).and_modify(|x|
                        x.push(Box::new(word.clone())));
                    return Box::new(word);
                }
            }
            '!' => {}
            '>' => {}
            '=' => {}
            _ => {}
        }

        //проверка на число
        if char::is_digit(self.peek, 10) {
            let mut result = 0;
            loop {
                result = 10 * result + char::to_digit(self.peek, 10).unwrap();
                self.read_char_or_return_eof(&mut for_scan);
                if !char::is_digit(self.peek, 10) {
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
                    //Вернем итератор на предыдущий символ, если текущий не слово
                    self.peek = for_scan.next_back().unwrap_or('\0');
                    self.index -= 1;
                    //
                    if self.words.get(&result).is_none() {
                        let word = Word::new(Tag::WORD, result.clone());
                        self.words.insert(result.clone(), word.clone());
                        self.lines.entry(self.line as usize).and_modify(|x|
                            x.push(Box::new(word.clone())));
                        return Box::new(word);
                    }
                }
            }
        }
        return Box::new(Word::new(Tag::WORD, "EOF".to_string()));
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
        let tr = Word::new(Tag::WORD, "id".to_owned());
        l.reserve(tr);
        assert_eq!(l.words.get("TRUE").unwrap().to_owned().tag, Tag::TRUE);
        assert_eq!(l.words.get("id").unwrap().to_owned().tag, Tag::WORD);
    }

    #[test]
    fn scan_string_test() {
        use super::*;
        let mut l = Lexer::new();
        while l.line != -1 {
            let a = l.scan("123 234 <= hello\n now be //    asdasdfasd\nas<\n let 23 /*vasya*\\");
            println!("get token: {:?}", a.deref());
            if &a.value()[..] == "EOF" {
                break;
            }
        }

        let w1: Box<dyn Token> = Box::new(Word::new(Tag::WORD, "now".to_owned()));
        let w2: Box<dyn Token> = Box::new(Word::new(Tag::WORD, "be".to_owned()));
        assert_eq!(l.lines.get(&2_usize).unwrap(),
                   &Box::new(vec![w1, w2]));
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