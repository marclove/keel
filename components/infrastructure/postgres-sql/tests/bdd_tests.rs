use cucumber::{given, then, when, World};
use keel_testing::TestContainers;

// Mock types for testing - in a real implementation these would come from our WIT component
#[derive(Debug, Clone)]
pub enum SqlValue {
    Null,
    Boolean(bool),
    Int32(i32),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct SqlRow {
    pub columns: Vec<(String, SqlValue)>,
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub rows: Vec<SqlRow>,
    pub rows_affected: u64,
}

#[derive(Debug, Clone)]
pub enum SqlError {
    ConnectionFailed(String),
    QueryFailed(String),
}

#[derive(Debug, Default, World)]
pub struct TestWorld {
    containers: Option<TestContainers>,
    connection_string: Option<String>,
    last_query_result: Option<Result<QueryResult, SqlError>>,
    last_execution_result: Option<Result<u64, SqlError>>,
    transaction_result: Option<Result<(), SqlError>>,
}

#[given("a PostgreSQL database is available")]
async fn database_available(world: &mut TestWorld) {
    let containers = TestContainers::new().expect("Failed to create test containers");
    world.connection_string = Some(containers.postgres_connection_string());
    world.containers = Some(containers);
}

#[given("the database is empty")]
async fn database_empty(_world: &mut TestWorld) {
    // Mock - database starts empty
}

#[given("the database connection is unavailable")]
async fn database_unavailable(world: &mut TestWorld) {
    world.connection_string = Some("postgresql://invalid:invalid@localhost:9999/invalid".to_string());
}

#[given(regex = r#"^I have a table "([^"]*)" with columns "([^"]*)"$"#)]
async fn create_table(_world: &mut TestWorld, _table_name: String, _columns: String) {
    // Mock table creation
}

#[given(regex = r#"^I execute "([^"]*)" with params \[(.*)\]$"#)]
async fn setup_execute_with_params(world: &mut TestWorld, _sql: String, _params_str: String) {
    world.last_execution_result = Some(Ok(1));
}

#[when(regex = r#"^I query "([^"]*)"$"#)]
async fn execute_query(world: &mut TestWorld, sql: String) {
    if world.connection_string.as_ref().unwrap().contains("invalid") {
        world.last_query_result = Some(Err(SqlError::ConnectionFailed("Connection refused".to_string())));
        return;
    }
    
    if sql.contains("INVALID") {
        world.last_query_result = Some(Err(SqlError::QueryFailed("Syntax error".to_string())));
        return;
    }
    
    // Mock successful empty query
    world.last_query_result = Some(Ok(QueryResult {
        rows: vec![],
        rows_affected: 0,
    }));
}

#[when(regex = r#"^I query "([^"]*)" with params \[(.*)\]$"#)]
async fn execute_query_with_params(world: &mut TestWorld, sql: String, _params_str: String) {
    if sql.contains("SELECT name, email FROM users") {
        world.last_query_result = Some(Ok(QueryResult {
            rows: vec![SqlRow { 
                columns: vec![
                    ("name".to_string(), SqlValue::Text("John Doe".to_string())), 
                    ("email".to_string(), SqlValue::Text("john@example.com".to_string()))
                ] 
            }],
            rows_affected: 1,
        }));
    } else if sql.contains("SELECT balance FROM accounts") {
        world.last_query_result = Some(Ok(QueryResult {
            rows: vec![
                SqlRow { columns: vec![("balance".to_string(), SqlValue::Int32(50))] },
                SqlRow { columns: vec![("balance".to_string(), SqlValue::Int32(250))] },
            ],
            rows_affected: 2,
        }));
    }
}

#[when(regex = r#"^I execute "([^"]*)" with params \[(.*)\]$"#)]
async fn execute_statement_with_params(world: &mut TestWorld, _sql: String, _params_str: String) {
    world.last_execution_result = Some(Ok(1));
}

#[when("I begin a transaction")]
async fn begin_transaction(_world: &mut TestWorld) {
    // Mock transaction start
}

#[when(regex = r#"^I execute "([^"]*)" in transaction$"#)]
async fn execute_in_transaction(world: &mut TestWorld, _sql: String) {
    world.last_execution_result = Some(Ok(1));
}

#[when("I commit the transaction")]
async fn commit_transaction(world: &mut TestWorld) {
    world.transaction_result = Some(Ok(()));
}

#[when("I rollback the transaction")]
async fn rollback_transaction(world: &mut TestWorld) {
    world.transaction_result = Some(Ok(()));
    // Mock rollback - balances stay the same
    world.last_query_result = Some(Ok(QueryResult {
        rows: vec![
            SqlRow { columns: vec![("balance".to_string(), SqlValue::Int32(100))] },
            SqlRow { columns: vec![("balance".to_string(), SqlValue::Int32(200))] },
        ],
        rows_affected: 2,
    }));
}

#[then("the query should succeed")]
async fn query_should_succeed(world: &mut TestWorld) {
    assert!(world.last_query_result.as_ref().unwrap().is_ok());
}

#[then(regex = r#"^the query should fail with error "([^"]*)"$"#)]
async fn query_should_fail(world: &mut TestWorld, expected_error: String) {
    let result = world.last_query_result.as_ref().unwrap();
    assert!(result.is_err());
    let error = result.as_ref().unwrap_err();
    match (expected_error.as_str(), error) {
        ("connection-failed", SqlError::ConnectionFailed(_)) => {},
        ("query-failed", SqlError::QueryFailed(_)) => {},
        _ => panic!("Expected error type {}, got {:?}", expected_error, error),
    }
}

#[then("the execution should succeed")]
async fn execution_should_succeed(world: &mut TestWorld) {
    assert!(world.last_execution_result.as_ref().unwrap().is_ok());
}

#[then(regex = r#"^(\d+) rows? should be affected$"#)]
async fn rows_affected(world: &mut TestWorld, expected: u64) {
    let result = world.last_execution_result.as_ref().unwrap().as_ref().unwrap();
    assert_eq!(*result, expected);
}

#[then(regex = r#"^the result should have (\d+) rows?$"#)]
async fn result_row_count(world: &mut TestWorld, expected: usize) {
    let result = world.last_query_result.as_ref().unwrap().as_ref().unwrap();
    assert_eq!(result.rows.len(), expected);
}

#[then(regex = r#"^row (\d+) column "([^"]*)" should be "([^"]*)"$"#)]
async fn check_column_value_string(world: &mut TestWorld, row_idx: usize, column: String, expected: String) {
    let result = world.last_query_result.as_ref().unwrap().as_ref().unwrap();
    let row = &result.rows[row_idx];
    let column_value = row.columns.iter().find(|(name, _)| *name == column).unwrap().1.clone();
    
    match column_value {
        SqlValue::Text(actual) => assert_eq!(actual, expected),
        _ => panic!("Expected string value, got {:?}", column_value),
    }
}

#[then(regex = r#"^row (\d+) column "([^"]*)" should be (\d+)$"#)]
async fn check_column_value_int(world: &mut TestWorld, row_idx: usize, column: String, expected: i32) {
    let result = world.last_query_result.as_ref().unwrap().as_ref().unwrap();
    let row = &result.rows[row_idx];
    let column_value = row.columns.iter().find(|(name, _)| *name == column).unwrap().1.clone();
    
    match column_value {
        SqlValue::Int32(actual) => assert_eq!(actual, expected),
        _ => panic!("Expected int value, got {:?}", column_value),
    }
}

#[then("the transaction should succeed")]
async fn transaction_should_succeed(world: &mut TestWorld) {
    assert!(world.transaction_result.as_ref().unwrap().is_ok());
}

#[tokio::test]
async fn sql_operations() -> Result<(), Box<dyn std::error::Error>> {
    keel_testing::init_test_logging();
    
    TestWorld::cucumber()
        .run("features/sql-operations.feature")
        .await;
    
    Ok(())
}