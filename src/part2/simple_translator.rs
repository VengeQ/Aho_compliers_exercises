use std::char;
use std::str::Chars;

struct SimpleTranslator<'a> {
    lookahead: char,
    string: Chars<'a>,
    result: String,
}

impl<'a> SimpleTranslator<'a> {
    pub fn from_string(input: &'a str) -> Self {
        SimpleTranslator {
            string: input.chars(),
            lookahead: '\0',
            result: "".to_owned(),
        }
    }

    pub fn expr(&mut self) -> Result<String, String> {
        self.lookahead = self.string.next().unwrap_or_else(|| '\n');
        match self.term() {
            Ok(_) => {}
            Err(e) => return Err(e)
        };
        loop {
            if self.lookahead == '-' {
                if let Err(e) = self.matcher('-'){
                    return Err(e)
                }
                if let Err(e) = self.term(){
                    return Err(e)
                }
                self.result += "-";
            } else if self.lookahead == '+' {
                if let Err(e) = self.matcher('+'){
                    return Err(e)
                }
                if let Err(e) = self.term(){
                    return Err(e)
                }
                self.result += "+";
            } else if self.lookahead == '\n'{
                break;
            } else {
                return Err("Expected + or -".to_owned())
            }
        }
        Ok(self.result.to_owned())
    }

    #[allow(dead_code)]
    pub fn show_result(&self) -> String {
        self.result.to_owned()
    }

    fn term(&mut self) -> Result<char, String> {
        if self.lookahead.is_digit(10) {
            self.result += &self.lookahead.to_string();
            self.matcher(self.lookahead).unwrap();
            Ok(self.lookahead)
        } else {
            Err("Expected digit".to_owned())
        }
    }


    fn matcher(&mut self, t: char) -> Result<(), String> {
        if self.lookahead == t {
            self.lookahead = match self.string.next() {
                Some(x) => x,
                None => '\n'
            };
            Ok(())
        } else {
            Err("Expected + or -".to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_translator_test() {
        use super::SimpleTranslator;
        let mut st = SimpleTranslator::from_string("2+3-4");
        assert_eq!(st.expr(), Ok("23+4-".to_owned()));
        st = SimpleTranslator::from_string("2+3-4-9");
        assert_eq!(st.expr(), Ok("23+4-9-".to_owned()));
        st = SimpleTranslator::from_string("2+3--4-9");
        assert_eq!(st.expr(), Err("Expected digit".to_owned()));
        st = SimpleTranslator::from_string("2+3-54-9");
        assert_eq!(st.expr(), Err("Expected + or -".to_owned()));
    }
}