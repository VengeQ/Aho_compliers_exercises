
extern crate regex;

mod simple_translator;
mod lexer;

use std::borrow::Cow;
use std::time::Duration;
#[allow(dead_code)]
enum Types {
    List,
    Term,
}
#[allow(dead_code)]
fn arithmetic(string: String) -> i32 {
    let mut stack: Vec<char> = Vec::new();
    let trimmed_string: Vec<char> = string.clone().chars().filter(|x| *x != ' ').collect();
    let mut is_digits = true;
    let mut result = 0;
    for value in &trimmed_string {
        match value {
            _ if value.is_digit(10) => stack.push(*value),
            '-' => {
                if is_digits {
                    let tempo = stack.pop().unwrap().to_string().parse::<i32>().unwrap();
                    result = stack.pop().unwrap().to_string().parse::<i32>().unwrap() - tempo;
                    is_digits = false;
                } else {
                    result = stack.pop().unwrap().to_string().parse::<i32>().unwrap() - result;
                }
            }
            '+' => {
                if is_digits {
                    result = stack.pop().unwrap().to_string().parse::<i32>().unwrap() + stack.pop().unwrap().to_string().parse::<i32>().unwrap();
                    is_digits = false;
                } else {
                    result += stack.pop().unwrap().to_string().parse::<i32>().unwrap();
                }
            }

            _ => panic!("Incorrect string")
        }
        //println!("{}",result);
    }
    result
}
#[allow(dead_code)]
fn harmonic_recursive(n: i64) -> f64 {
    fn go(n: f64, limit: i64, accum: f64) -> f64 {
        if n > limit as f64 { accum } else {
            go(n + 1_f64, limit, accum + 1_f64 / n)
        }
    }
    go(1_f64, n, 0_f64)
}
#[allow(dead_code)]
fn harmonic(n: i64) -> f64 {
    let mut result = 0_f64;
    for i in 1..n {
        result += 1_f64 / (i as f64)
    };
    result
}
#[allow(dead_code)]
fn translate_to_postfix(string: &str) -> String {
    let regex = regex::Regex::new(r"(\d+)([-+])(\d+)").unwrap();
    let regex_2 = regex::Regex::new(r"(?P<first>([\d\s]+)([-+]))(?P<symbol>[-+])(?P<second>\d+)").unwrap();
    let mut is_match = true;
    let mut after = string.to_owned();

    while is_match {
        let before = after.to_owned();
        std::thread::sleep(Duration::from_millis(500));

        after = match regex.replace(&after, "$1 $3 $2") {
            Cow::Borrowed(x) => x.to_owned(),
            Cow::Owned(x) => x
        };

        after = match regex_2.replace_all(&after, "$first $second $symbol") {
            Cow::Borrowed(x) => x.to_owned(),
            Cow::Owned(x) => x,
        };
        println!("before: {}\nafter: {}", before.clone(), after.clone());
        if after == before { is_match = false };
    };
    after
}
#[allow(dead_code)]
fn rome(number: i32) -> String {
    let mut result ="".to_owned();

    result += match number%100/10 {
        0 => "",
        1 => "X",
        2 =>"XX",
        3 =>"XXX",
        4 =>"XL",
        5 =>"L",
        6 =>"LX",
        7 => "LXX",
        8 =>"LXXX",
        9 => "XC",
        _ => unreachable!()
    };
    result += match number%10{
        0 =>"",
        1 =>"|",
        2 =>"||",
        3 =>"|||",
        4 =>"|V",
        5 =>"V",
        6 =>"V|",
        7 =>"V||",
        8 =>"V|||",
        9 =>"|X",
        _ => unreachable!()
    };
    result

}

#[cfg(test)]
mod tests {
    #[test]
    fn arithmetic_simple_test() {
        use super::arithmetic;
        assert_eq!(arithmetic("263-+".to_owned()), 5);
        assert_eq!(arithmetic("26378-++-".to_owned()), 2 - (6 + (3 + (7 - 8))));
    }

    #[test]
    #[should_panic(expected = "Incorrect string")]
    fn arithmetic_incorrect_chars_in_input_test() {
        use super::arithmetic;
        assert_eq!(arithmetic("26a-+".to_owned()), 5);
    }

    #[test]
    fn translate_to_postfix_test() {
        use super::translate_to_postfix;
        assert_eq!(translate_to_postfix("22+4"), String::from("22 4 +"));
        assert_eq!(translate_to_postfix("22+4-14"), String::from("22 4 + 14 -"));
        assert_eq!(translate_to_postfix("22+4-14+24"), String::from("22 4 + 14 - 24 +"));
    }

    #[test]
    fn harmonic_test_smoke() {
        use super::harmonic;
        println!("{}", harmonic(10e8 as i64));
    }

    #[test]
    fn rome_test(){
        use super::rome;
        assert_eq!(&rome(25),"XXV");
        assert_eq!(&rome(78),"LXXV|||");
    }
}
