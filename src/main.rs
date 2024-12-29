use std::io;

mod db;
use db::Database;

fn ask_noun(db: &Database) {
    let noun = db.get_noun();

    loop {
        println!("___ {}?", noun.word);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("I/O error");

        if guess.trim().to_lowercase() == noun.article {
            println!("correct");
            db.mark_correct(&noun);
            return;
        }

        println!("wrong, try again");
    }
}

fn main() {
    let db = Database::init();
    loop {
        ask_noun(&db);
    }
}
