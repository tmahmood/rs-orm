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

    // #connect to database
    // connect to database using given connection string
    pub fn new(dsn:&str) -> Database {
        match Connection::connect(dsn, TlsMode::None) {
            Ok(conn) => Database{conn: conn},
            Err(e) => panic!("Failed to connect database: {:?}", e)
        }
    }

    pub fn exec_direct(&self, sql:&str) -> Result<u64, Error> {
        self.conn.execute(&sql, &[])
    }

    // Insertion
    // generate Insert SQL insert the given data
    // @data is reference of slice containing all the data
    pub fn insert(&self, table:&str, cols:String, data: &[&ColumnType]) -> i32 {
        // NOTE: postgres driver uses $1..$n to bind variables,
        // we make the binding string here, there must be
        // a better way to do it?
        // TODO: Find a better way to generate placeholder string
        let mut i = 0;
        let placeholders:Vec<String> = data.iter()
            .map(|&_| { i += 1; format!("${}", i) })
            .collect();
        // build the query string, join method seems faster than
        // string concat
        let sql = format!("insert into {} ({}) values({}) returning id",
                table, cols, placeholders.join(", "));
        // prepare statement
        let stmt = self.conn.prepare(&sql).unwrap();
        // execute query
        // insert query in postgres returns the insert id
        // which we like to have, so check and return
        let r = match stmt.query(data) {
            Ok(rows) => {
                let row = rows.iter().next().unwrap();
                row.get(0)
            }, Err(e) => {
                println!("Error Inserting: {:?}", e);
                0
            }
        };
        r
    }

    pub fn update(&self, table:&str, cols:Vec<String>, id:i32, data:&[&ColumnType])
            -> bool {
        let mut i = 0;
        let placeholders:Vec<String> = cols.iter()
            .map(|ref c| { i += 1; format!("{} = ${}", c, i) })
            .collect();
        let sql = format!("update {} set {} where id = {}", table,
                            &placeholders.join(", "), id);
        // Execute query
        let stmt = self.conn.prepare(&sql).unwrap();
        let r = match stmt.query(data) {
            Ok(rows) => { true },
            Err(e) => {
                println!("Error Inserting: {:?}", e);
                false
            }
        };
        r
    }

    pub fn select_by_id(&self, table:&str, id:i32) -> Rows {
        let mut sql: String;
        sql = format!("select * from {} where id={}", table, id);
        self.conn.query(&sql, &[]).unwrap()
    }

    pub fn select_all(&self, table:&str, limit:usize) -> Rows {
        let mut sql: String;
        if limit != 0 {
            sql = format!("select * from {} limit {}", table, limit);
        } else {
            sql = format!("select * from {}", table);
        }
        println!("{}", sql);
        // Execute query
        self.conn.query(&sql, &[]).unwrap()
    }

    pub fn delete(&self, table:&str, id:i32) -> Result<u64, Error> {
        let sql = format!("delete from {} where id = {}", table, id);
        self.conn.execute(&sql, &[])
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
// join: t3 t1.id = t3.t1_id
// where:

// pub struct Name {
//     pub label: String,
//     pub name: String
// }
//
// pub struct Join {
//     pub _table: String,
//     pub on: Vec<Condition>
// }
//
// pub struct QueryBuilder {
//     pub _tables: Vec<Name>,
//     pub _columns: Vec<Name>,
//     pub _join: Vec<Join>,
// }
//
