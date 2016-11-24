use lib::pgsql::{Database, ColumnType, DatabaseError, DataRow, DataRows};

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
    fn columns_as_csv() -> String;
    fn fill(row:&Row) -> T;
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

    pub fn delete(&self, name:&str, id:i32) -> Result<u64, DatabaseError> {
        self.database.delete(name, id)
    }

    pub fn find_all<'a, T:DataTraits<T>>(&self, name:&str, limit:usize) -> Vec<T> {
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
macro_rules! model {
    (struct $name:ident {$($fname:ident : $ftype:ty),*}, $table:expr) => {
        #[derive(Debug)]
        pub struct $name {
            pub id:i32,
            $(pub $fname : $ftype),*
        }

        impl DataTraits<$name> for $name {
            fn fill(row:&Row) -> $name {
                $name {
                        id: row.get("id"),
                        $($fname: row.get(stringify!($fname))),*
                    }
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

            fn set_id(&mut self, id:i32){
                self.id = id;
            }

            fn name() -> &'static str {
                $table
            }
        }
    }
}

