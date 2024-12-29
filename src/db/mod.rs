use chrono::Utc;
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
}

impl Database {
    fn create_nouns_table(connection: &Connection) {
        let query = "
          create table nouns(
            id integer primary key,
            article text,
            word text,
            last_correct integer);
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
            where last_correct is null
            order by random()
            limit 1
        ";
        let mut statement = self.connection.prepare(query).unwrap();

        statement.next().unwrap();

        Noun {
            id: statement.read("id").unwrap(),
            article: statement.read("article").unwrap(),
            word: statement.read("word").unwrap(),
        }
    }

    pub fn mark_correct(&self, noun: &Noun) {
        let ts = Utc::now().timestamp();

        let q = format!(
            "update nouns set last_correct = {} where id = {}",
            ts, noun.id
        );
        self.connection.execute(q).unwrap();
    }
}
