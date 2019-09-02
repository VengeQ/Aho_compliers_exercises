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
/// ws -> (\n\r\t)+
///

#[derive(Debug, PartialEq)]
struct Token(String);

impl Token{
    fn token(&self) -> String{
        self.0.to_owned()
    }
    fn from_str(s:&str) -> Self{
       Token(s.to_string())
    }
}

#[derive(Debug)]
struct Lexer{
    lexemeBegin:usize,
    forward:usize
}

impl<'a> Lexer{
    fn new() -> Self{
        Lexer{lexemeBegin:0,forward:0}
    }


    fn scan_test(& mut self, input:String) -> Token{
        Token("hello".to_owned())
    }
    fn scan(& mut self, input:String) -> Token{
        let re = regex::Regex::new("[\n\r\t]+").unwrap();
        self.forward = input[0..input.len()].chars().take_while(|x|{
            *x=='\n'|| *x=='\r' || *x =='\t' || *x ==' '
        } ).count();
        Token(self.forward.to_string())
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn token_test(){
        use super::Token;
        let t = Token::from_str(">=");
        assert_eq!(t.token(), ">=");
    }

    #[test]
    fn lexer_smoke_test(){
        use super::Token;
        use super::Lexer;
        let t = Token::from_str(">=");
        let mut lexer = Lexer::new();

        assert_eq!(lexer.scan_test(">=".to_string()),Token::from_str("hello"));
        assert_eq!(lexer.scan("\t\t\t >=".to_string()),Token::from_str("4"));
    }
}