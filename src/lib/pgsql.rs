use postgres::{Connection, TlsMode};
use postgres::types::{ToSql,FromSql};
use postgres::rows::{Rows, Row};
use postgres::stmt::Statement;
use postgres::error::Error;

pub type DatabaseError = Error;
pub type ColumnType = ToSql;
pub struct Database {
    pub conn: Connection,
}

// Trait to fill up data from given row.
pub trait FillStruct <T>{
    fn fill(row:Row)->T;
    fn empty()->T;
}

impl Database {
    pub fn new(dsn:&str) -> Database {
        match Connection::connect(dsn, TlsMode::None) {
            Ok(conn) => Database{conn: conn},
            Err(e) => panic!("Failed to connect database: {:?}", e)
        }
    }

    pub fn insert(&self, sql_orig:&str, data: &[&ColumnType]) -> i32 {
        // in case of insert we can get the last inserted id
        // by adding "returning id" at the end
        let sql = &vec![sql_orig, "returning id"].join(" ");
        let stmt = self.conn.prepare(sql).unwrap();
        return match stmt.query(data) {
            Ok(rows) => {
                let row = rows.iter().next().unwrap();
                row.get(0)
            },
            Err(e) => {
                println!("{:?}", e);
                0
            }
        };
    }

    pub fn clear(&self, table:&str) -> Result<u64, Error>  {
        let sql = format!("ALTER SEQUENCE {}_id_seq RESTART WITH 1", table);
        let m = self.statement(&sql);
        let state = m.execute(&[]);

        let sql = format!("truncate {} cascade", table);
        let m = self.statement(&sql);
        m.execute(&[])
    }

    pub fn statement(&self, sql:&str) -> Statement {
        return match self.conn.prepare(sql) {
            Ok(stmt) => stmt,
            Err(_) => panic!("Failed to prepare SQL")
        };
    }

    // convert row to the given struct
    pub fn get_row_object<T:FillStruct<T>>(&self,
                                           sql:&'static str,
                                           data: &[&ToSql]) -> T {
        let stmt = self.statement(sql);
        return match stmt.query(data) {
            Ok(rows) => {
                if rows.len() == 0 {
                    T::empty()
                } else {
                    let row = rows.iter().next().unwrap();
                    T::fill(row)
                }
            },
            Err(_) => T::empty()
        };
    }

}

