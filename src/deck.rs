use std::collections::HashSet;
use std::result::Result;

const DECK_SIZE: usize = 8;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Noun {
    pub article: &'static str,
    pub word: &'static str,
}

const NOUNS: [Noun; 8] = [
    Noun {
        article: "the",
        word: "apple",
    },
    Noun {
        article: "a",
        word: "banana",
    },
    Noun {
        article: "the",
        word: "carrot",
    },
    Noun {
        article: "a",
        word: "dog",
    },
    Noun {
        article: "the",
        word: "elephant",
    },
    Noun {
        article: "a",
        word: "fish",
    },
    Noun {
        article: "the",
        word: "guitar",
    },
    Noun {
        article: "a",
        word: "house",
    },
];

#[derive(Debug)]
pub struct Deck {
    noun_indices: HashSet<usize>,
    current_noun_idx: usize,
}

fn next_index(indices: &HashSet<usize>) -> Option<&usize> {
    indices.iter().next()
}

impl Deck {
    pub fn new() -> Deck {
        let mut nouns_indices = HashSet::new();

        for i in 0..DECK_SIZE {
            nouns_indices.insert(i);
        }

        let cur_idx = *next_index(&nouns_indices).unwrap();

        Deck {
            noun_indices: nouns_indices,
            current_noun_idx: cur_idx,
        }
    }

    pub fn get_current_noun(&self) -> &Noun {
        &NOUNS[self.current_noun_idx]
    }

    pub fn goto_next_noun(&mut self) -> Result<(), ()> {
        self.noun_indices.remove(&self.current_noun_idx);

        match next_index(&self.noun_indices) {
            Some(idx) => {
                self.current_noun_idx = *idx;
                Ok(())
            }
            None => Err(()),
        }
    }
}
