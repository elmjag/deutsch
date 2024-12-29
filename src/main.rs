use std::io;

mod db;
mod last;

use db::Database;
use last::LastIDs;

fn ask_noun(db: &Database, last_ids: &mut LastIDs) {
    let noun = db.get_noun(last_ids.get_ids());
    let mut misstake = false;

    loop {
        println!("___ {}?", noun.word);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("I/O error");

        if guess.trim().to_lowercase() == noun.article {
            println!("correct");
            break;
        }

        misstake = true;
        println!("wrong, try again");
    }

    match misstake {
        true => db.decrease_level(&noun),
        false => db.increase_level(&noun),
    }

    last_ids.add_id(noun.get_id());
}

fn main() {
    let mut last_noun_ids = LastIDs::new();
    let db = Database::init();

    loop {
        ask_noun(&db, &mut last_noun_ids);
    }
}
