use lib::pgsql::{Database, ColumnType, DatabaseError, DataRow, DataRows};
use std::collections::{HashSet, HashMap};

pub type FieldType = ColumnType;
pub type Rows<'a> = DataRows<'a>;
pub type Row<'a> = DataRow<'a>;
pub struct StorageProvider {
    database: Database,
}

pub trait DataTraits<T> {
    fn name() -> &'static str;
    fn columns() -> Vec<String>;
    fn data(&self) -> Vec<&ColumnType>;
    fn set_id(&mut self, i32);
    fn get_id(&self) -> i32;
    fn columns_as_csv() -> String;
    fn fill(row:&Row) -> T;
    fn changed(&self) -> Vec<String>;
    fn changed_data(&self) -> HashMap<String, &FieldType> ;
}

impl StorageProvider {
    pub fn new(database:Database) -> StorageProvider {
        StorageProvider{ database: database }
    }

    pub fn insert<T:DataTraits<T>>(&self, obj:&mut T) {
        let id;
        {
            let v = obj.data();
            id = self.database.insert(T::name(), T::columns_as_csv(), &v);
        }
        obj.set_id(id);
    }

    pub fn clear(&self, table:&str) -> Result<u64, DatabaseError> {
        self.database.clear(table)
    }

    pub fn update<T:DataTraits<T>>(&self, obj:&T) -> i32 {
        let changes = obj.changed_data();
        let cols:Vec<String> = Vec::new();
        let data:Vec<&FieldType> = Vec::new();
        for (col, change) in &changes {
            cols.push(col);
            data.push(change);
        }
        self.database.update(T::name(), cols, obj.get_id(), &data)
    }

    pub fn delete(&self, name:&str, id:i32) -> Result<u64, DatabaseError> {
        self.database.delete(name, id)
    }

    pub fn find<T:DataTraits<T>>(&self, name:&str, id:i32) -> T {
        let mut rows:DataRows = self.database.select_by_id(name, id);
        // should never be more than 1
        assert!(rows.len() < 2);
        let row = rows.iter().next().unwrap();
        T::fill(&row)
    }

    pub fn find_all<T:DataTraits<T>>(&self, name:&str, limit:usize) -> Vec<T> {
        let mut data_rows:DataRows = self.database.select_all(name, limit);
        let mut list:Vec<T> = Vec::new();
        for row in data_rows.iter() {
            list.push(T::fill(&row));
        }
        list
    }
}

// Generates standard model struct and implements
// traits required to work with it
// NOTE: models will only provide access to data and ways to
// set data, nothing else
// TODO: Keep track of changes [HOW?]
macro_rules! model {
    (struct $name:ident {$($fname:ident : $ftype:ty),*}, $table:expr) => {

        #[derive(Debug)]
        pub struct $name {
            pub id:i32,
            changed: HashSet<String>,
            $(pub $fname : $ftype),*
        }

        impl $name {
            pub fn new($($fname:$ftype),*) -> $name {
                $name {
                        id: 0,
                        changed: HashSet::new(),
                        $($fname: $fname),*
                }
            }

            $(
            pub fn $fname(&mut self, val:$ftype) -> &mut $name {
                self.$fname = val;
                self.changed.insert(String::from(stringify!($fname)));
                self
            }
            )*
        }

        impl DataTraits<$name> for $name {
            fn fill(row:&Row) -> $name {
                $name {
                        id: row.get("id"),
                        changed: HashSet::new(),
                        $($fname: row.get(stringify!($fname))),*
                    }
            }

            fn changed(&self)-> Vec<String> {
                let mut lst = Vec::new();
                for item in &self.changed {
                    lst.push(item.clone());
                }
                lst
            }

            fn columns() -> Vec<String> {
                vec![$(String::from(stringify!($fname))), *]
            }

            fn columns_as_csv() -> String {
                let t = vec![$(String::from(stringify!($fname))), *];
                t.join(", ")
            }

            fn data(&self) -> Vec<&FieldType> {
                vec![ $(&self.$fname), *]
            }

            fn changed_data(&self) -> HashMap<String, &FieldType> {
                let mut lst:HashMap<String, &FieldType> = Vec::new();
                $(
                    let cname = stringify!($fname);
                    if self.changed.contains(cname) {
                        lst[String::from(cname)] = &self.$fname;
                    }
                )*
                lst
            }

            fn set_id(&mut self, id:i32){
                self.id = id;
            }

            fn get_id(&self) -> i32 {
                self.id
            }

            fn name() -> &'static str {
                $table
            }
        }
    }
}

