#[allow(dead_code)]
#[allow(unused_imports)]
use lib::storageprovider::{StorageProvider, FieldType, DataTraits, Row};
use lib::pgsql::Database;
use ::dotenv::dotenv;
use ::std::env;
use ::chrono::Local;
use std::collections::{HashSet, HashMap};

model!( struct Survey {
                    title: String,
                    duration: i32,
                    start_date: i64,
                    active: bool
                }, "surveys");

model!( struct QuestionSet {
                    title: String
                }, "questionsets");

model!( struct Option {
                    label: String
                }, "options");

model!( struct OptionSet {
                    label: String
                }, "optionsets");

pub fn connect() -> StorageProvider {
    dotenv().ok();
    let dsn = env::var("DATABASE_URL").unwrap_or(String::from("Failed"));
    StorageProvider::new(Database::new(&dsn))
}

#[test]
fn test_db_crud() {
    let storage = connect();
    // clean table
    let _ = storage.clear(Survey::name());
    let _ = storage.clear(QuestionSet::name());

    // create new entry
    let time = Local::now().timestamp();
    let mut survey = Survey::new( format!("Oct 2016 Survey"), 8,
                                  time, true);
    assert_eq!(true, storage.insert(&mut survey));
    // check if id was set
    assert!(survey.id > 0);
    // save again, but fail. because it already exists
    assert_eq!(false, storage.insert(&mut survey));

    // loading, check if all value matches
    let mut new_one:Survey = storage.find(Survey::name(), survey.id);
    // check if all are set correctly
    assert_eq!(new_one.title, "Oct 2016 Survey");
    assert_eq!(new_one.duration, 8);
    assert_eq!(new_one.active, true);
    assert_eq!(new_one.start_date, time);
    // update data
    new_one.title(String::from("Nov 2016 Survey"))
           .duration(6)
           .active(false);
    assert_eq!(true, storage.update(&new_one));
    // load it again
    let first_one:Survey = storage.find(Survey::name(), 1);
    assert_eq!(first_one.title, "Nov 2016 Survey");
    assert_eq!(first_one.duration, 6);
    assert_eq!(first_one.active, false);

    // clear the table, this will also reset related sequence
    let mut ids = Vec::new();
    for i in 1..11 {
        let mut questionset = QuestionSet::new(format!("{}. Agree or Disagree?", i));
        assert_eq!(true, storage.insert(&mut questionset));
        assert!(questionset.id == i);
        ids.push(i);
    }
    // there should be 10 QuestionSet
    let qsets:Vec<QuestionSet> = storage.find_all(QuestionSet::name(), 0);
    assert_eq!(10, qsets.len());
    assert_eq!(qsets[0].title, "1. Agree or Disagree?");

    let qsets:Vec<QuestionSet> = storage.find_all(QuestionSet::name(), 2);
    assert_eq!(2, qsets.len());
    // now we delete by ids
    for i in 0..ids.len() {
        let cnt = storage.delete(QuestionSet::name(), ids[i as usize])
                         .expect("Failed");
        assert_eq!(1, cnt);
    }
}
