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
    // clear the table
    // ----------------------------------------------
    let _ = storage.clear("surveys");

    // inserting data
    // ----------------------------------------------
    let mut survey = Survey {
                        id: 0, title: format!("Oct 2016 Survey"),
                        duration: 8, start_date: Local::now().timestamp(),
                        active: true, };
    println!("{:?}", survey);
    storage.create(&survey);
    // insert_query!(storage, &mut survey);
    // survey.id = storage.create(survey);
    // finding data
    // ----------------------------------------------
    // let id = survey.get("id").unwrap();
    // println!("{}", id);
    //let survey = storage.find_by_id("surveys", id);
    // #
}
