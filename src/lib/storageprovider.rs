use lib::pgsql::Database;
use std::collections::HashMap;

#[derive(Debug)]
pub enum FieldValue {
    IntegerField(i32),
    StringField(&'static str),
    BoolField(bool),
    FloatField(f32),
}

pub struct StorageProvider {
    database: Database,
}

impl StorageProvider {
    pub fn new(database:Database) -> StorageProvider {
        StorageProvider{ database: database }
    }

    pub fn create(&self, table:&str, data:&mut HashMap<&'static str, FieldValue>){
        // join up columns
        let keys:Vec<String> = data.keys().map(|&x| String::from(x)).collect();
        let mut vars = vec![];
        for i in 0..data.len() {
            vars.push(format!("${}", i))
        }
        let mut sql = format!("insert into {} ({}) values ({})",
                                table, keys.join(", "), vars.join(", "));
        data.insert("id", FieldValue::IntegerField(10));
    }

    pub fn clear(&self, table:&str){
        let sql = format!("truncate {} cascade", table);
        println!("{}", sql);
    }

    pub fn find_by_id(&self, table:&str, id:FieldValue)
            -> HashMap<&'static str, FieldValue> {
        HashMap::new()
    }
}
