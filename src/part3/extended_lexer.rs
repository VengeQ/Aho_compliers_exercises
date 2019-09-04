use std::str::Chars;
use std::cmp::min;
use std::fmt::{Debug, Formatter, Error};
use std::collections::HashMap;

///Шаблоны токенов для этого горе-лексера
///
/// digit ->[0-9]
/// digits -> digit+
/// number ->digits(.digits)?([eE][+-]?digits)?
/// letter ->[A-Za-z]
/// id -> letter(letter|digits)*
/// if -> IF
/// then -> THEN
/// else -> ELSE
/// relop -> >= |<= |< |== |> |!=
/// ws -> (\n\r\t\s)+
///
#[derive(Debug, PartialEq, Clone)]
struct Token(String);

impl Token {
    fn token(&self) -> String {
        self.0.to_owned()
    }
    fn from_str(s: &str) -> Self {
        Token(s.to_string())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Buffer {
    A,
    B,
}

const BUFFERSIZE:usize=2048;

struct Lexer {
    buffer_a: [char; BUFFERSIZE],
    buffer_b: [char; BUFFERSIZE],
    lexeme_begin: usize,
    forward: usize,
    buffer_idx: usize,
    tokens: HashMap<String, regex::Regex>,
    buffer: Buffer,
}

impl Debug for Lexer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let string = self.cur_buffer().to_vec().into_iter().take_while(|x| *x != '\0').collect::<Vec<char>>();
        write!(f, "{:?}", string)
    }
}


impl<'a> Lexer {
    fn new() -> Self {
        Lexer { lexeme_begin: 0, forward: 0, buffer_a: ['\0'; BUFFERSIZE], buffer_b: ['\0'; BUFFERSIZE], buffer_idx: 0, tokens: Lexer::init_regex(), buffer: Buffer::A }
    }

    fn cur_buffer(&self) -> [char; BUFFERSIZE] {
        if self.buffer == Buffer::A {
            self.buffer_a
        } else {
            self.buffer_b
        }
    }

    fn init_regex() -> HashMap<String, regex::Regex> {
        let mut hash: HashMap<String, regex::Regex> = HashMap::new();
        hash.insert("number".to_owned(), regex::Regex::new(r"^(\d+(\.\d+)?(([Ee][+-])?\d+)?)").unwrap());
        hash.insert("id".to_owned(), regex::Regex::new(r"^[a-zA-Z]([a-zA-Z0-9])*").unwrap());
        hash.insert("relop".to_owned(), regex::Regex::new(r"^([!=]=|<[=]??|>[=]??)").unwrap());
        hash
    }

    fn init_buffer(&mut self, input: String) {
        let mut input_as_chars = input.chars();
        //Последний элемент eof
        println!("{}", &input[self.buffer_idx..].len());
        println!("{}", &(self.buffer_a.len() - 1));

        for i in 0..self.cur_buffer().len() {
            self.buffer_a[i] = input_as_chars.next().unwrap_or('\0');
        }
        if &input[self.buffer_idx..].len() > &(self.buffer_a.len() - 1) {
            let mut idx = self.cur_buffer().len() - 2;
            while !self.buffer_a[idx].is_whitespace() && self.buffer_a[idx] != '\0' {
                self.buffer_a[idx] = '\0';
                idx -= 1;
            }
            self.buffer_idx = idx;
        }
    }

    fn change_buffer(&mut self, input: &str) {
        if self.buffer == Buffer::A {
            if &input[self.buffer_idx..].len() > &0 {
                self.buffer == Buffer::B;
                println!("change!");
            }
        } else {}
    }

    fn scan_test(&mut self, input: String) -> Token {
        Token("hello".to_owned())
    }

    fn is_eof(&mut self) -> bool {
        if self.lexeme_begin >= self.cur_buffer().len() {
            true
        } else {
            if self.cur_buffer()[self.lexeme_begin] == '\0'// && self.buffer == Buffer::A && self.forward == self.buffer_a.len()
            {
                self.clear();
                true
            } else { false }
        }
    }
    pub fn scan(&mut self, input: &str) -> Option<Token> {
        println!("{}", self.cur_buffer()[self.lexeme_begin].to_string());
        //Убрать пробелы
        if self.cur_buffer()[self.lexeme_begin].is_whitespace() {
            self.forward = self.cur_buffer()[self.lexeme_begin..self.cur_buffer().len()].iter().take_while(|&x| {
                *x == '\n' || *x == '\r' || *x == '\t' || *x == ' '
            }).count() + self.forward;
            self.lexeme_begin = self.forward;
            println!("{}", self.lexeme_begin);
        }

        let text = &self.cur_buffer()[self.lexeme_begin..].to_vec().iter().collect::<String>()[..];

        if let Some(m) = self.tokens.get("number").unwrap().find(text) {
            self.forward += m.end();
            self.lexeme_begin = self.forward;
            return Some(Token(m.as_str().to_owned()));
        }

        if let Some(m) = self.tokens.get("id").unwrap().find(text) {
            self.forward += m.end();
            self.lexeme_begin = self.forward;
            return match m.as_str() {
                "IF" => Some(Token("if:IF".to_owned())),
                "THEN" => Some(Token("then:THEN".to_owned())),
                "ELSE" => Some(Token("else:ELSE".to_owned())),
                _ => Some(Token(m.as_str().to_owned()))
            };
        }

        if let Some(m) = self.tokens.get("relop").unwrap().find(text) {
            self.forward += m.end();
            self.lexeme_begin = self.forward;

            return Some(Token(m.as_str().to_owned()));
        }

        if self.is_eof() {
            self.change_buffer(&input);
            None
        } else {
            panic!("Unexpected token at {}", self.lexeme_begin)
        }
    }
    fn clear(&mut self) {
        self.lexeme_begin = 0;
        self.buffer_idx = 0;
        self.forward = 0;
        self.buffer_a = ['\0'; 2048];
        self.buffer_b = ['\0'; 2048];
        self.buffer = Buffer::A
    }

    fn update_buffer() {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use super::Token;
    use super::Lexer;
    use std::time::Duration;

    #[test]
    fn token_test() {
        let t = Token::from_str(">=");
        assert_eq!(t.token(), ">=");
    }

    #[test]
    fn lexer_smoke_test() {
        let mut lexer = Lexer::new();
        assert_eq!(lexer.scan_test(">=".to_string()), Token::from_str("hello"));
    }

    #[test]
    fn lexer_eof_test() {
        let mut lexer = Lexer::new();
        let string_for_scan = "as==112.23\n  142e-1>= s241  2341 hello<me".to_owned();
        //let string_for_scan = "42e-1>= s241".to_owned();
        lexer.init_buffer(string_for_scan.clone());
        let mut a = lexer.scan(&string_for_scan[..]);
        while a != None {
            println!("{:?}", &a);
            std::thread::sleep(Duration::from_millis(1000));
            a = lexer.scan(&string_for_scan[..]);
        }
    }


    #[test]
    fn lexer_test() {
        let mut lexer = Lexer::new();
        let string_for_scan = "  234\n   counter=1\nIF counter ==1\n THEN counter = 2\n ELSE counter =3\n".to_owned();
        lexer.init_buffer(string_for_scan.clone());
        let a = lexer.scan(&string_for_scan[..]);
        println!("{:?}", a);
    }
}