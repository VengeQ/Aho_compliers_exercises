use std::collections::HashMap;
use std::ops::Deref;
use core::fmt;
use std::fmt::Debug;

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Clone)]
enum Tag {
    NUM,
    WORD,
    FALSE,
    TRUE,
}

trait Token {
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

impl<'t> Token for Num {
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
}


impl Lexer {
    pub fn new() -> Self {
        let mut lexer = Lexer {
            index: 0,
            line: 1,
            peek: ' ',
            words: Default::default(),
        };
        lexer.words.insert("TRUE".to_owned(), Word::new(Tag::TRUE, "TRUE".to_owned()));
        lexer.words.insert("FALSE".to_owned(), Word::new(Tag::FALSE, "FALSE".to_owned()));
        lexer
    }

    fn reserve(&mut self, word: Word) {
        self.words.insert(word.value(), word);
    }

    pub fn read_char_or_return_eof(& mut self, input:&str) ->char{
        let mut for_scan = input.split_at(self.index).1.chars();
        self.peek=for_scan.next().unwrap_or_else('\0');
        if !self.peek=='\0'{
            self.index+=1;
        }
        self.peek
    }


    pub fn scan(&mut self, input: &str) -> Box<dyn Token> {
        if self.index>=input.len(){
            return Box::new(Word::new(Tag::WORD, "EOF".to_string()));
        }
        let mut for_scan = input.split_at(self.index).1.chars();
        loop {
            self.index += 1;
            self.peek = for_scan.next().unwrap_or_else(|| '\0');
            match self.peek {
                ' ' | '\t' => {}
                '\n' => {
                    self.line += 1;
                }
                '\0' => {
                    self.line = -1;
                    return Box::new(Word::new(Tag::WORD, "EOF".to_string()));
                }
                _ => break
            }
        }

        if self.peek == '/' {
            self.index += 1;
            self.peek = for_scan.next().unwrap_or_else(|| '\0');
            if self.peek == '/' {
                loop {
                    self.index += 1;
                    self.peek = for_scan.next().unwrap_or('\0');
                    if self.peek == '\0' {
                        break;
                    }
                    if self.peek == '\n'{
                        self.index+=1;
                        self.peek = for_scan.next().unwrap_or('\0');
                        println!("{}", self.peek);
                        break;
                    }
                }
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

        if char::is_alphabetic(self.peek) {
            let mut result = "".to_owned();
            loop {
                self.index += 1;
                result.push(self.peek);
                self.peek = for_scan.next().unwrap_or_else(|| '\0');
                if !char::is_alphabetic(self.peek) {
                    if self.words.get(&result).is_none() {
                        let word = Word::new(Tag::WORD, result.clone());
                        self.words.insert(result.clone(), word.clone());
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
            let a = l.scan("123 234  hello\n now be //    asdasdfasd\nas");
            println!("get token: {:?}", a.deref());
            if &a.value()[..] == "EOF" {
                break;
            }
        }
    }
}