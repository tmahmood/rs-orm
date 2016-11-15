#[macro_use]

use lib::pgsql::{Database, ColumnType, DatabaseError};

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

    pub fn create<T:DataTraits<T>>(&self, table:&mut T) -> i32 {
        let sql = table.insert_sql();
        let vals = table.values();
        self.database.insert(&sql, &vals)
    }

    pub fn insert<'a>(&self, sql:&str, data:&Vec<&'a ColumnType>) -> i32 {
        self.database.insert(&sql, data)
    }

    pub fn clear(&self, table:&str) -> Result<u64, DatabaseError> {
        let sql = format!("truncate {} cascade", table);
        let m = self.database.statement(&sql);
        m.execute(&[])
    }
}

// Generates standard model struct and implements
// traits required to work with it
macro_rules! model {
    (struct $name:ident {$($fname:ident : $ftype:ty),*}, $x:expr) => {
        #[derive(Debug)]
        pub struct $name {
            pub id:i32,
            $(pub $fname : $ftype),*
        }

        impl DataTraits<$name> for $name {
            fn insert(&self) -> String {
                 let l = vec![$(stringify!($fname)), *];
                 let mut i = 0;
                 let x:Vec<String> = l.iter().map(|&_| {
                        i += 1;
                        format!("${}", i)
                 }).collect();
                 format!("insert into {} ({}) values({})", $x,
                         l.join(", "), x.join(","))
            }

            fn values<'a>(&'a self) -> Vec<&'a FieldType> {
                vec![ $(&self.$fname),* ]
            }

            fn set_id(&mut self, id:i32) {
                self.id = id
            }
        }
    }
}

