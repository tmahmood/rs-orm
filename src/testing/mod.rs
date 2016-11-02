use std::collections::HashMap;
use lib::storageprovider::{StorageProvider,FieldValue};
use lib::pgsql::Database;
use ::dotenv::dotenv;
use ::std::env;


#[test]
fn test_storage_provider() {
    dotenv().ok();
    let dsn = env::var("DATABASE_URL").unwrap_or(String::from("Failed"));
    let storage = StorageProvider::new(Database::new(&dsn));
    // lets define behaviour
    //
    // clear the table
    // ----------------------------------------------
    storage.clear("surveys");
    // #
    // inserting data
    // ----------------------------------------------
    let mut survey = HashMap::new();
    survey.insert("title",      FieldValue::StringField("Oct 2016 Survey"));
    survey.insert("duration",   FieldValue::IntegerField(8));
    survey.insert("start_date", FieldValue::StringField("2016-10-21"));
    survey.insert("active",     FieldValue::BoolField(true));
    // #
    storage.create("surveys", &mut survey);
    // finding data
    // ----------------------------------------------
    let id = survey.get("id").unwrap();
    println!("{:?}", id);
    //let survey = storage.find_by_id("surveys", id);
    // #
}
