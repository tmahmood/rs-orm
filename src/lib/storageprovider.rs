#[macro_use]

use lib::pgsql::{Database, ColumnType, DatabaseError};

pub type FieldType = ColumnType;
pub struct StorageProvider {
    database: Database,
}

pub trait DataTraits <T>{
    fn insert(&mut self, &StorageProvider);
    fn name() -> &'static str;
}

impl StorageProvider {
    pub fn new(database:Database) -> StorageProvider {
        StorageProvider{ database: database }
    }

    pub fn insert<'a>(&self, sql:&str, data:&Vec<&'a ColumnType>) -> i32 {
        self.database.insert(&sql, data)
    }

    pub fn clear(&self, table:&str) -> Result<u64, DatabaseError> {
        self.database.clear(table)
    }
}

// Generates standard model struct and implements
// traits required to work with it
macro_rules! model {
    (struct $name:ident {$($fname:ident : $ftype:ty),*}, $table:expr) => {
        #[derive(Debug)]
        pub struct $name {
            pub id:i32,
            $(pub $fname : $ftype),*
        }
        impl DataTraits<$name> for $name {
            fn insert(&mut self, storage:&StorageProvider) {
                 let columns = vec![$(stringify!($fname)), *];
                 let mut i = 0;
                 // can we clean this up a bit?
                 let placeholders:Vec<String> = columns.iter()
                                                       .map(|&_| { i += 1; format!("${}", i) })
                                                       .collect();
                 let sql = format!("insert into {} ({}) values({})", $table, columns.join(", "),
                                                                     placeholders.join(","));
                 // we need some error handling here and return status
                 let id = storage.insert(&sql, &vec![ $(&self.$fname),* ]);
                 // assign id, now we know we
                 // haved saved the survey
                 self.id = id;
            }
            fn name() -> &'static str {
                $table
            }
        }
    }
}

