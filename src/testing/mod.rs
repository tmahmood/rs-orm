use lib::storageprovider::{StorageProvider, FieldType, DataTraits};
use lib::pgsql::Database;
use ::dotenv::dotenv;
use ::std::env;
use ::chrono::Local;

macro_rules! make_fn {
    ($x:expr, $y:expr, $z:expr) => {
         fn insert_sql(&self) -> String {
             format!("insert into {} ({}) values({})", $x, $y, $z)
         }
     }
}

#[derive(Debug)]
pub struct Survey {
    pub id: i32,
    pub title: String,
    pub duration: i32,
    pub start_date: i64,
    pub active: bool,
}

impl DataTraits<Survey> for Survey {
    make_fn!("surveys", "title, duration, start_date, active", "$1, $2, $3, $4");
    fn values<'a>(&'a self) -> Vec<&'a FieldType> {
        vec![&self.title, &self.duration, &self.start_date, &self.active]
    }

    fn set_id(&mut self, id:i32) {
        self.id = id;
    }
}

#[test]
fn test_storage_provider() {
    dotenv().ok();
    let dsn = env::var("DATABASE_URL").unwrap_or(String::from("Failed"));
    let storage = StorageProvider::new(Database::new(&dsn));
    // lets define behaviour
    //
    // clear the table
    // ----------------------------------------------
    let _ = storage.clear("surveys");
    // #
    // inserting data
    // ----------------------------------------------
    let mut survey = Survey {
                        id: 0, title: format!("Oct 2016 Survey"),
                        duration: 8, start_date: Local::now().timestamp(),
                        active: true,
                     };
    println!("{:?}", survey);
    survey = storage.create(survey);

    // finding data
    // ----------------------------------------------
    // let id = survey.get("id").unwrap();
    // println!("{}", id);
    //let survey = storage.find_by_id("surveys", id);
    // #
}
