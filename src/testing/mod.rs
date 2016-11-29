#[allow(dead_code)]
#[allow(unused_imports)]
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
    let mut survey = Survey::new(
                        format!("Oct 2016 Survey"), 8,
                        Local::now().timestamp(), true);
    // save it!
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
    let all_surveys:Vec<Survey> = storage.find_all(Survey::name(), 0);
    assert_eq!(11, all_surveys.len());
    assert_eq!(all_surveys[0].title, "Oct 2016 Survey");
    // now we delete by ids
    for i in 0..ids.len() {
        let cnt = storage.delete(Survey::name(), ids[i as usize])
                         .expect("Failed");
        assert_eq!(1, cnt);
    }
    // now select all again
    let final_one:Vec<Survey> = storage.find_all(Survey::name(), 0);
    // should be 1
    assert_eq!(1, final_one.len());
    // and it should be the first one we inserted outside of loop
    assert_eq!(final_one[0].title, "Oct 2016 Survey");
    {
        // select one only
        // find a survey
        let mut first_one:Survey = storage.find(Survey::name(), 1);
        assert_eq!(first_one.title, "Oct 2016 Survey");
        assert_eq!(first_one.id, 1);
        // TODO: update it
        first_one.title(String::from("Nov 2016 Survey"))
                 .active(true);
        assert_eq!(true, storage.update(&first_one));
    }
    // load it again
    let first_one:Survey = storage.find(Survey::name(), 1);
    assert_eq!(first_one.title, "Nov 2016 Survey");
    // delete it
    let cnt = storage.delete(Survey::name(), first_one.id);
    assert_eq!(cnt.unwrap(), 1);
    // now select all again
    let no_rows:Vec<Survey> = storage.find_all(Survey::name(), 0);
    // should be 0
    assert_eq!(0, no_rows.len());
}
