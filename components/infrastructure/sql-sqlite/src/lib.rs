wit_bindgen::generate!({
    world: "sql-adapter",
    path: "../../../wit",
});

use exports::keel::infrastructure::sql::{Guest, GuestTransaction, SqlValue, QueryResult, SqlError, Transaction};

pub struct TransactionImpl {
    _id: String,
}

struct Component;

impl Guest for Component {
    type Transaction = TransactionImpl;
    
    fn query(_sql: String, _params: Vec<SqlValue>) -> Result<QueryResult, SqlError> {
        todo!("query not yet implemented")
    }

    fn execute(_sql: String, _params: Vec<SqlValue>) -> Result<u64, SqlError> {
        todo!("execute not yet implemented")
    }

    fn begin_transaction() -> Result<Transaction, SqlError> {
        todo!("begin_transaction not yet implemented")
    }
}

impl GuestTransaction for TransactionImpl {
    fn commit(&self) -> Result<(), SqlError> {
        todo!("commit not yet implemented")
    }

    fn rollback(&self) -> Result<(), SqlError> {
        todo!("rollback not yet implemented")
    }
}

export!(Component);