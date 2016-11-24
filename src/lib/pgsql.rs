use postgres::{Connection, TlsMode};
use postgres::types::{ToSql,FromSql};
use postgres::rows::{Rows, Row};
use postgres::stmt::Statement;
use postgres::error::Error;

pub type DatabaseError = Error;
pub type ColumnType = ToSql;
pub type DataRows<'a> = Rows<'a>;
pub type DataRow<'a> = Row<'a>;
pub struct Database {
    pub conn: Connection,
}

impl Database {

    pub fn new(dsn:&str) -> Database {
        match Connection::connect(dsn, TlsMode::None) {
            Ok(conn) => Database{conn: conn},
            Err(e) => panic!("Failed to connect database: {:?}", e)
        }
    }

    pub fn insert(&self, table:&str, cols:String, data: &[&ColumnType]) -> i32{
        let mut i = 0;
        // postgres driver uses $1..$n to bind variables,
        // we make the binding string here, there must be
        // a better way to do it?
        let placeholders:Vec<String> = data.iter()
            .map(|&_| { i += 1; format!("${}", i) })
            .collect();
        // build the query string
        let sql = format!("insert into {} ({}) values({}) returning id",
                table, cols, placeholders.join(", "));
        // prepare statement
        let stmt = self.conn.prepare(&sql).unwrap();
        // execute query
        return match stmt.query(data) {
            Ok(rows) => {
                let row = rows.iter().next().unwrap();
                row.get(0)
            }, Err(e) => {
                println!("{:?}", e);
                0
            }
        };
    }

    pub fn select_all(&self, table:&str, limit:usize) -> Rows {
        let mut sql: String;
        if limit != 0 {
            sql = format!("select * from {} limit by {}", table, limit);
        } else {
            sql = format!("select * from {}", table);
        }
        // prepare statement
        self.conn.query(&sql, &[]).unwrap()
    }

    pub fn delete(&self, table:&str, id:i32) -> Result<u64, Error> {
        let sql = format!("delete from {} where id = $1", table);
        let statement = self.statement(&sql);
        statement.execute(&[&id])
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
}

//
// tables: t1, t2
// columns: t1.*, t2.name, t3.cat
// join t3 t1.id = t3.t1_id
// where:


