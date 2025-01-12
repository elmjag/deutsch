use sqlite::{self, Connection};
use std::path::Path;

mod nouns;
use nouns::NOUNS;

const DB_FILE: &str = "./db";

pub struct Database {
    connection: Connection,
}

pub struct Noun {
    id: i64,
    pub article: String,
    pub word: String,
    level: i64,
}

impl Database {
    fn create_nouns_table(connection: &Connection) {
        let query = "
          create table nouns(
            id integer primary key,
            article text,
            word text,
            level integer default 0);
        ";
        connection.execute(query).unwrap();

        for (article, word) in NOUNS {
            let q = format!(
                "insert into nouns(article, word) values('{}', '{}');",
                article, word
            );
            connection.execute(q).unwrap();
        }
    }

    pub fn init() -> Database {
        let db_exists = Path::new(DB_FILE).exists();
        let connection = sqlite::open("./db").unwrap();

        if !db_exists {
            Database::create_nouns_table(&connection);
        }

        Database {
            connection: connection,
        }
    }

    pub fn get_noun(&self) -> Noun {
        let query = "
          select * from nouns
            order by level, random()
            limit 1
        ";
        let mut statement = self.connection.prepare(query).unwrap();

        statement.next().unwrap();

        Noun {
            id: statement.read("id").unwrap(),
            article: statement.read("article").unwrap(),
            word: statement.read("word").unwrap(),
            level: statement.read("level").unwrap(),
        }
    }

    fn update_level(&self, noun: &Noun, new_level: i64) {
        let q = format!(
            "update nouns set level = {} where id = {}",
            new_level, noun.id
        );
        self.connection.execute(q).unwrap();
    }

    pub fn decrease_level(&self, noun: &Noun) {
        let new_level = if noun.level > 0 {
            noun.level - 1
        } else {
            noun.level
        };

        self.update_level(noun, new_level);
    }

    pub fn increase_level(&self, noun: &Noun) {
        self.update_level(noun, noun.level + 1);
    }
}
