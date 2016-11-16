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
    survey.insert(&storage);
    assert!(survey.id == 1);
    // add lot more
    for i in 1..100 {
        survey.title = format!("Test Survey {}", i);
        survey.duration = 6;
        survey.active = Local::now().timestamp() % 2 == 0;
        survey.insert(&storage);
        assert!(survey.id == i + 1);
    }

    // storage.delete(Survey::name(), survey.id);
    // add another one
    // it should have an id
    // finding data
    // ----------------------------------------------
    // println!("{}", id);
    // let survey = storage.find_by_id("surveys", id);
    // #
}
