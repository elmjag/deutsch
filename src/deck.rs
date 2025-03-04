use std::result::Result;

mod nouns;
use nouns::NOUNS;

const DECK_SIZE: usize = 4;

#[derive(Debug)]
pub struct Noun {
    pub article: &'static str,
    pub word: &'static str,
}

impl Noun {
    fn new(a: &'static str, b: &'static str) -> Noun {
        Noun {
            article: a,
            word: b,
        }
    }
}

#[derive(Debug)]
pub struct Deck {
    nouns: Vec<Noun>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut nouns = Vec::new();

        for i in 0..DECK_SIZE {
            let (a, w) = NOUNS[i];
            nouns.push(Noun::new(a, w));
        }

        Deck { nouns: nouns }
    }

    pub fn get_current_noun(&self) -> &Noun {
        self.nouns.last().unwrap()
    }

    pub fn goto_next_noun(&mut self) -> Result<(), ()> {
        self.nouns.pop();

        if self.nouns.is_empty() {
            return Err(());
        }

        Ok(())
    }
}
