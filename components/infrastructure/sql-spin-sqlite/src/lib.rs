//! Spin-backed SQLite implementation of the `sql` WIT interface.
//! Uses Spin's host-provided SQLite for performance and simplicity.

use spin_sdk::sqlite::{Connection, QueryResult as SpinQueryResult, Value as SpinValue};

wit_bindgen::generate!({
    world: "sql-adapter",
    path: "../../../wit",
});

use exports::keel::infrastructure::sql::{self as wit_sql};

// Map WIT sql-value to Spin SQLite Value
fn to_spin_value(v: &wit_sql::SqlValue) -> Result<SpinValue, wit_sql::SqlError> {
    use wit_sql::SqlValue as W;
    Ok(match v {
        W::Null => SpinValue::Null,
        W::Boolean(b) => SpinValue::Integer(if *b { 1 } else { 0 }),
        W::Int32(i) => SpinValue::Integer(*i as i64),
        W::Int64(i) => SpinValue::Integer(*i),
        W::Float32(f) => SpinValue::Real(*f as f64),
        W::Float64(f) => SpinValue::Real(*f),
        W::Text(s) => SpinValue::Text(s.clone()),
        W::Bytes(b) => SpinValue::Blob(b.clone()),
        W::Timestamp(ts) => SpinValue::Integer(*ts),
        W::Uuid(u) => SpinValue::Text(u.clone()),
    })
}

fn map_err<E: std::error::Error>(e: E, kind: &'static str) -> wit_sql::SqlError {
    match kind {
        "connection" => wit_sql::SqlError::ConnectionFailed(e.to_string()),
        "query" => wit_sql::SqlError::QueryFailed(e.to_string()),
        "transaction" => wit_sql::SqlError::TransactionFailed(e.to_string()),
        _ => wit_sql::SqlError::QueryFailed(e.to_string()),
    }
}

fn values_from(params: &[wit_sql::SqlValue]) -> Result<Vec<SpinValue>, wit_sql::SqlError> {
    params.iter().map(to_spin_value).collect()
}

fn row_to_wit(columns: &[String], row_values: &[SpinValue]) -> wit_sql::SqlRow {
    let cols = columns
        .iter()
        .cloned()
        .zip(row_values.iter().cloned())
        .map(|(name, v)| {
            let val = match v {
                SpinValue::Null => wit_sql::SqlValue::Null,
                SpinValue::Integer(i) => wit_sql::SqlValue::Int64(i),
                SpinValue::Real(f) => wit_sql::SqlValue::Float64(f),
                SpinValue::Text(s) => wit_sql::SqlValue::Text(s),
                SpinValue::Blob(b) => wit_sql::SqlValue::Bytes(b),
            };
            (name, val)
        })
        .collect();
    wit_sql::SqlRow { columns: cols }
}

struct Adapter;

impl wit_sql::Guest for Adapter {
    type Transaction = Transaction;

    fn query(sql: String, params: Vec<wit_sql::SqlValue>) -> Result<wit_sql::QueryResult, wit_sql::SqlError> {
        exec_query(&sql, &params)
    }

    fn execute(sql: String, params: Vec<wit_sql::SqlValue>) -> Result<u64, wit_sql::SqlError> {
        exec_execute(&sql, &params)
    }

    fn begin_transaction() -> Result<wit_sql::Transaction, wit_sql::SqlError> {
        let conn = Connection::open_default().map_err(|e| map_err(e, "connection"))?;
        conn.execute("BEGIN", &[]).map_err(|e| map_err(e, "transaction"))?;
        Ok(wit_sql::Transaction::new(Transaction { conn }))
    }
}

struct Transaction {
    conn: Connection,
}

impl wit_sql::GuestTransaction for Transaction {
    fn query(&self, sql: String, params: Vec<wit_sql::SqlValue>) -> Result<wit_sql::QueryResult, wit_sql::SqlError> {
        exec_query_on(&self.conn, &sql, &params)
    }

    fn execute(&self, sql: String, params: Vec<wit_sql::SqlValue>) -> Result<u64, wit_sql::SqlError> {
        exec_execute_on(&self.conn, &sql, &params)
    }

    fn commit(&self) -> Result<(), wit_sql::SqlError> {
        self.conn
            .execute("COMMIT", &[])
            .map_err(|e| map_err(e, "transaction"))?;
        Ok(())
    }

    fn rollback(&self) -> Result<(), wit_sql::SqlError> {
        self.conn
            .execute("ROLLBACK", &[])
            .map_err(|e| map_err(e, "transaction"))?;
        Ok(())
    }
}

// Export the component entry points
export!(Adapter);

fn exec_query(sql: &str, params: &[wit_sql::SqlValue]) -> Result<wit_sql::QueryResult, wit_sql::SqlError> {
    let conn = Connection::open_default().map_err(|e| map_err(e, "connection"))?;
    let values = values_from(params)?;
    let qr: SpinQueryResult = conn
        .execute(sql, values.as_slice())
        .map_err(|e| map_err(e, "query"))?;
    let mut out_rows = Vec::new();
    for row in &qr.rows {
        out_rows.push(row_to_wit(&qr.columns, &row.values));
    }
    Ok(wit_sql::QueryResult { rows: out_rows, rows_affected: 0 })
}

fn exec_execute(sql: &str, params: &[wit_sql::SqlValue]) -> Result<u64, wit_sql::SqlError> {
    let conn = Connection::open_default().map_err(|e| map_err(e, "connection"))?;
    exec_execute_on(&conn, sql, params)
}

fn exec_query_on(conn: &Connection, sql: &str, params: &[wit_sql::SqlValue]) -> Result<wit_sql::QueryResult, wit_sql::SqlError> {
    let values = values_from(params)?;
    let qr: SpinQueryResult = conn
        .execute(sql, values.as_slice())
        .map_err(|e| map_err(e, "query"))?;
    let mut out_rows = Vec::new();
    for row in &qr.rows {
        out_rows.push(row_to_wit(&qr.columns, &row.values));
    }
    Ok(wit_sql::QueryResult { rows: out_rows, rows_affected: 0 })
}

fn exec_execute_on(conn: &Connection, sql: &str, params: &[wit_sql::SqlValue]) -> Result<u64, wit_sql::SqlError> {
    let values = values_from(params)?;
    conn.execute(sql, values.as_slice()).map_err(|e| map_err(e, "query"))?;
    Ok(0)
}
