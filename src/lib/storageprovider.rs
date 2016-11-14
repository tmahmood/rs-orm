use lib::pgsql::{Database, ColumnType, DatabaseError};
use std::collections::HashMap;

pub type FieldType = ColumnType;
pub struct StorageProvider {
    database: Database,
}

pub trait DataTraits <T>{
    fn insert_sql(&self)->String;
    fn values<'a>(&'a self)-> Vec<&'a ColumnType>;
    fn set_id(&mut self,i32);
}

impl StorageProvider {
    pub fn new(database:Database) -> StorageProvider {
        StorageProvider{ database: database }
    }

    pub fn create<T:DataTraits<T>>(&self, table:T) -> T {
        let mut sql;
        let mut vals;
        sql = table.insert_sql();
        vals = table.values();
        let id = self.database.insert(&sql, &vals);
        table.set_id(id);
        table
    }

    pub fn clear(&self, table:&str) -> Result<u64, DatabaseError> {
        let sql = format!("truncate {} cascade", table);
        let m = self.database.statement(&sql);
        m.execute(&[])
    }

    pub fn find_by_id(&self, table:&str, id:i32) -> HashMap<&'static str, &str> {
        HashMap::new()
    }
}
