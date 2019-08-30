use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Default, Eq, PartialEq)]
struct Env {
    table: HashMap<String, String>,
    previous: Box<Option<Env>>,
}


impl Env {
    pub fn new() -> Self {
        Env::default()
    }

    pub fn insert(&mut self, key: &str, sym: String) {
        if self.table.get(key).is_none() {
            self.table.insert(key.to_owned(), sym);
        } else {
            panic!("This symbol already exists in current namespace");
        };
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match self.table.get(key) {
            Some(x) => Some(x.to_owned()),
            None => match self.previous.deref() {
                Some(env) => env.get(key),
                None => None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_env_test() {
        use super::Env;
        let e = Env::new();
        let prev = e.previous;
        let expected: Option<Env> = None;
        assert_eq!((*prev), None::<Env>);
        assert_eq!(e.table.len(), 0);
    }

    #[test]
    fn insert_get_test() {
        use super::Env;
        let mut e = Env::new();
        e.insert("x", "Char".to_owned());
        e.insert("y", "Int".to_owned());
        assert_eq!(e.get("x").unwrap(), "Char".to_owned());
        assert_eq!(e.get("s"), None::<String>);
    }
    #[test]
    #[should_panic(expected = "This symbol already exists in current namespace")]
    fn get_test_panic() {
        use super::Env;
        let mut e = Env::new();
        e.insert("x", "Char".to_owned());
        e.insert("x", "Int".to_owned());
    }
}