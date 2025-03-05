use std::result::Result;

mod nouns;
use nouns::NOUNS;

mod db;
use db::Database;

const DB_FILE: &str = "./deutsch.db";

const DECK_SIZE: usize = 16;

#[derive(Debug)]
pub struct Noun {
    pub article: &'static str,
    pub word: &'static str,
    index: usize,
    level: u32,
}

impl Noun {
    fn new(article: &'static str, word: &'static str, index: usize, level: u32) -> Noun {
        Noun {
            article,
            word,
            index,
            level,
        }
    }
}

#[derive(Debug)]
pub struct Deck {
    nouns: Vec<Noun>,
    db: Database,
}

impl Deck {
    pub fn new() -> Result<Deck, String> {
        let db = match Database::connect(DB_FILE) {
            Ok(d) => d,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        let noun_data = match db.get_nouns(DECK_SIZE) {
            Ok(indices) => indices,
            Err(err) => return Err(err.to_string()),
        };

        let mut nouns = Vec::new();

        for (idx, level) in noun_data {
            let (article, word) = NOUNS[idx];
            nouns.push(Noun::new(article, word, idx, level));
        }

        let deck = Deck {
            nouns: nouns,
            db: db,
        };

        Ok(deck)
    }

    pub fn get_current_noun(&self) -> &Noun {
        self.nouns.last().unwrap()
    }

    pub fn increase_level(&self, noun: &Noun) -> Result<(), String> {
        let r = self.db.update_level(noun.index, noun.level + 1);
        match r {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn decrease_level(&self, noun: &Noun) -> Result<(), String> {
        if noun.level == 0 {
            /* already at lowest possible level */
            return Ok(());
        }

        let r = self.db.update_level(noun.index, noun.level - 1);
        match r {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn goto_next_noun(&mut self) -> Result<(), ()> {
        self.nouns.pop();

        if self.nouns.is_empty() {
            return Err(());
        }

        Ok(())
    }
}
