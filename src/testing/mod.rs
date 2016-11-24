#[macro_use]
use lib::storageprovider::{StorageProvider, FieldType, DataTraits};
use lib::pgsql::Database;
use ::dotenv::dotenv;
use ::std::env;
use ::chrono::Local;


use app::models::{Survey};

#[test]
fn test_storage_provider() {
    dotenv().ok();
    let dsn = env::var("DATABASE_URL").unwrap_or(String::from("Failed"));
    let storage = StorageProvider::new(Database::new(&dsn));
    // lets define behaviour
    // clear the table, this will also reset related sequence
    // ----------------------------------------------
    let _ = storage.clear(Survey::name());
    // inserting data
    // ----------------------------------------------
    assert_eq!("surveys", Survey::name());
    let mut survey = Survey {
                        id: 0, title: format!("Oct 2016 Survey"),
                        duration: 8, start_date: Local::now().timestamp(),
                        active: true, };
    // save it!
    // survey.insert(&storage);
    storage.insert(&mut survey);
    assert!(survey.id == 1);
    let mut ids = Vec::new();
    // add lot more
    for i in 1..11 {
        survey.title = format!("Test Survey {}", i);
        survey.duration = 6;
        survey.active = Local::now().timestamp() % 2 == 0;
        storage.insert(&mut survey);
        assert!(survey.id == i + 1);
        ids.push(i+1);
    }
    // there should be 11 survey
    let res:Vec<Survey> = storage.find_all(Survey::name(), 0);
    assert_eq!(11, res.len());
    assert_eq!(res[0].title, "Oct 2016 Survey");
    // find a survey
    // let survey = Survey::find(&storage, 1);
    // delete it
    // storage.delete(survey);
    // add another one
    // it should have an id
    // finding data
    // ----------------------------------------------
    // println!("{}", id);
    // let survey = storage.find_by_id("surveys", id);
    // #
    for i in 0..ids.len() {
        let cnt = storage.delete(Survey::name(), ids[i as usize]).expect("Failed");
        assert_eq!(1, cnt);
    }
    let res:Vec<Survey> = storage.find_all(Survey::name(), 0);
    assert_eq!(1, res.len());
    assert_eq!(res[0].title, "Oct 2016 Survey");
}
