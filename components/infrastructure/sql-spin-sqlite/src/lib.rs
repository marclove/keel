#![cfg_attr(not(target_arch = "wasm32"), deny(unsafe_code))]
#![cfg_attr(target_arch = "wasm32", allow(unsafe_code))]
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code, unused_imports))]
//! Spin-backed SQLite implementation of the `sql` WIT interface.
//! Uses Spin's host-provided SQLite for performance and simplicity.

use spin_sdk::sqlite::{Connection, QueryResult as SpinQueryResult, Value as SpinValue};

#[macro_use]
mod bindings {
    #![allow(unsafe_code)]
    #![allow(unsafe_op_in_unsafe_fn)]
    #![allow(unused_attributes)]
    #![allow(clippy::empty_line_after_outer_attr)]
    wit_bindgen::generate!({
        world: "sql-adapter",
        path: "wit",
    });
}

use crate::bindings::exports::keel::infrastructure::sql::{self as wit_sql};

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

    fn query(
        sql: String,
        params: Vec<wit_sql::SqlValue>,
    ) -> Result<wit_sql::QueryResult, wit_sql::SqlError> {
        exec_query(&sql, &params)
    }

    fn execute(sql: String, params: Vec<wit_sql::SqlValue>) -> Result<u64, wit_sql::SqlError> {
        exec_execute(&sql, &params)
    }

    fn begin_transaction() -> Result<wit_sql::Transaction, wit_sql::SqlError> {
        let conn = Connection::open_default().map_err(|e| map_err(e, "connection"))?;
        conn.execute("BEGIN", &[])
            .map_err(|e| map_err(e, "transaction"))?;
        Ok(wit_sql::Transaction::new(Transaction { conn }))
    }
}

struct Transaction {
    conn: Connection,
}

impl wit_sql::GuestTransaction for Transaction {
    fn query(
        &self,
        sql: String,
        params: Vec<wit_sql::SqlValue>,
    ) -> Result<wit_sql::QueryResult, wit_sql::SqlError> {
        exec_query_on(&self.conn, &sql, &params)
    }

    fn execute(
        &self,
        sql: String,
        params: Vec<wit_sql::SqlValue>,
    ) -> Result<u64, wit_sql::SqlError> {
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
#[cfg(target_arch = "wasm32")]
bindings::export!(Adapter with_types_in bindings);

fn exec_query(
    sql: &str,
    params: &[wit_sql::SqlValue],
) -> Result<wit_sql::QueryResult, wit_sql::SqlError> {
    let conn = Connection::open_default().map_err(|e| map_err(e, "connection"))?;
    let values = values_from(params)?;
    let qr: SpinQueryResult = conn
        .execute(sql, values.as_slice())
        .map_err(|e| map_err(e, "query"))?;
    let mut out_rows = Vec::new();
    for row in &qr.rows {
        out_rows.push(row_to_wit(&qr.columns, &row.values));
    }
    Ok(wit_sql::QueryResult {
        rows: out_rows,
        rows_affected: 0,
    })
}

fn exec_execute(sql: &str, params: &[wit_sql::SqlValue]) -> Result<u64, wit_sql::SqlError> {
    let conn = Connection::open_default().map_err(|e| map_err(e, "connection"))?;
    exec_execute_on(&conn, sql, params)
}

fn exec_query_on(
    conn: &Connection,
    sql: &str,
    params: &[wit_sql::SqlValue],
) -> Result<wit_sql::QueryResult, wit_sql::SqlError> {
    let values = values_from(params)?;
    let qr: SpinQueryResult = conn
        .execute(sql, values.as_slice())
        .map_err(|e| map_err(e, "query"))?;
    let mut out_rows = Vec::new();
    for row in &qr.rows {
        out_rows.push(row_to_wit(&qr.columns, &row.values));
    }
    Ok(wit_sql::QueryResult {
        rows: out_rows,
        rows_affected: 0,
    })
}

fn exec_execute_on(
    conn: &Connection,
    sql: &str,
    params: &[wit_sql::SqlValue],
) -> Result<u64, wit_sql::SqlError> {
    let values = values_from(params)?;
    conn.execute(sql, values.as_slice())
        .map_err(|e| map_err(e, "query"))?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_spin_value_maps_all_variants() {
        use wit_sql::SqlValue as W;

        assert!(matches!(to_spin_value(&W::Null).unwrap(), SpinValue::Null));
        assert!(matches!(
            to_spin_value(&W::Boolean(true)).unwrap(),
            SpinValue::Integer(1)
        ));
        assert!(matches!(
            to_spin_value(&W::Boolean(false)).unwrap(),
            SpinValue::Integer(0)
        ));
        assert!(matches!(
            to_spin_value(&W::Int32(7)).unwrap(),
            SpinValue::Integer(7)
        ));
        assert!(matches!(
            to_spin_value(&W::Int64(9)).unwrap(),
            SpinValue::Integer(9)
        ));
        assert!(
            matches!(to_spin_value(&W::Float32(1.5)).unwrap(), SpinValue::Real(x) if (x-1.5).abs() < 1e-6)
        );
        assert!(
            matches!(to_spin_value(&W::Float64(2.5)).unwrap(), SpinValue::Real(x) if (x-2.5).abs() < 1e-12)
        );
        assert!(
            matches!(to_spin_value(&W::Text("hi".into())).unwrap(), SpinValue::Text(s) if s=="hi")
        );
        assert!(
            matches!(to_spin_value(&W::Bytes(vec![1,2])).unwrap(), SpinValue::Blob(b) if b==vec![1,2])
        );
        assert!(matches!(
            to_spin_value(&W::Timestamp(123)).unwrap(),
            SpinValue::Integer(123)
        ));
        assert!(
            matches!(to_spin_value(&W::Uuid("abc".into())).unwrap(), SpinValue::Text(s) if s=="abc")
        );
    }

    #[test]
    fn row_to_wit_roundtrip_basic() {
        let cols = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];
        let values = vec![
            SpinValue::Null,
            SpinValue::Integer(42),
            SpinValue::Real(3.5),
            SpinValue::Text("ok".into()),
            SpinValue::Blob(vec![9, 8, 7]),
        ];

        let row = row_to_wit(&cols, &values);
        assert_eq!(row.columns.len(), cols.len());
        for (i, (name, v)) in row.columns.iter().enumerate() {
            assert_eq!(name, &cols[i]);
            match (i, v) {
                (0, wit_sql::SqlValue::Null) => {}
                (1, wit_sql::SqlValue::Int64(n)) => assert_eq!(*n, 42),
                (2, wit_sql::SqlValue::Float64(f)) => assert!((f - 3.5).abs() < 1e-12),
                (3, wit_sql::SqlValue::Text(s)) => assert_eq!(s, "ok"),
                (4, wit_sql::SqlValue::Bytes(b)) => assert_eq!(b, &vec![9, 8, 7]),
                _ => panic!("unexpected mapping at index {}: {:?}", i, v),
            }
        }
    }
}
