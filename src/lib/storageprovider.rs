use lib::pgsql::Database;
use std::collections::HashMap;

pub struct StorageProvider {
    database: Database,
}

impl StorageProvider {
    pub fn new(database:Database) -> StorageProvider {
        StorageProvider{ database: database }
    }

    pub fn create(&self, table:&str, data:&mut HashMap<&'static str, &str>){
        // join up columns
        let keys:Vec<String> = data.keys().map(|&x| String::from(x)).collect();
        let vals = data.values().map(|&x| &x).collect();
        let mut vars = vec![];
        for i in 0..data.len() {
            vars.push(format!("${}", i));
        }
        let mut sql = format!("insert into {} ({}) values ({})",
                                table, keys.join(", "), vars.join(", "));
        self.database.insert(&sql, vals);
        data.insert("id", "10");
    }

    pub fn clear(&self, table:&str){
        let sql = format!("truncate {} cascade", table);
        println!("{}", sql);
    }

    pub fn find_by_id(&self, table:&str, id:i32) -> HashMap<&'static str, &str> {
        HashMap::new()
    }
}
