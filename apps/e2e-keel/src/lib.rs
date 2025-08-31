use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{http::{Method, Request, Response}, http_component, sqlite::{Connection, Value}};

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    ok: bool,
    data: Option<T>,
    error: Option<String>,
}

#[http_component]
fn handle_e2e(req: Request) -> Result<Response> {
    let path = req.path_and_query().unwrap_or("/");
    let method = req.method();
    match (method, path) {
        (m, "/setup") if *m == Method::Post => setup(),
        (m, "/users") if *m == Method::Post => create_user(req),
        (m, "/users") if *m == Method::Get => list_users(),
        (m, "/txn/commit") if *m == Method::Post => txn_commit(),
        (m, "/txn/rollback") if *m == Method::Post => txn_rollback(),
        _ => json(404, &ApiResponse::<()> { ok: false, data: None, error: Some("not found".into()) }),
    }
}

fn json<T: Serialize>(status: u16, val: &T) -> Result<Response> {
    let body = serde_json::to_vec(val)?;
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .build())
}

fn setup() -> Result<Response> {
    let db = Connection::open_default()?;
    db.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, email TEXT)", &[])?;
    db.execute("CREATE TABLE IF NOT EXISTS accounts (id INTEGER PRIMARY KEY AUTOINCREMENT, balance INTEGER)", &[])?;
    db.execute("DELETE FROM users", &[])?;
    db.execute("DELETE FROM accounts", &[])?;
    json(200, &ApiResponse::<()> { ok: true, data: None, error: None })
}

#[derive(Deserialize)]
struct NewUser { name: String, email: String }

fn create_user(req: Request) -> Result<Response> {
    let db = Connection::open_default()?;
    let body = req.body();
    let nu: NewUser = serde_json::from_slice(body)?;
    db.execute(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        &[Value::Text(nu.name), Value::Text(nu.email)],
    )?;
    json(200, &ApiResponse::<()> { ok: true, data: None, error: None })
}

#[derive(Serialize)]
struct UserOut { id: i64, name: String, email: String }

fn list_users() -> Result<Response> {
    let db = Connection::open_default()?;
    let qr = db.execute("SELECT id, name, email FROM users ORDER BY id", &[])?;
    let mut users = Vec::new();
    for row in &qr.rows {
        let id: i64 = row.get(0).unwrap_or_default();
        let name: &str = row.get(1).unwrap_or("");
        let email: &str = row.get(2).unwrap_or("");
        users.push(UserOut { id, name: name.to_string(), email: email.to_string() });
    }
    json(200, &ApiResponse { ok: true, data: Some(users), error: None })
}

fn txn_commit() -> Result<Response> {
    let db = Connection::open_default()?;
    db.execute("DELETE FROM accounts", &[])?;
    db.execute("INSERT INTO accounts (balance) VALUES (100), (200)", &[])?;
    db.execute("BEGIN", &[])?;
    db.execute("UPDATE accounts SET balance = balance - 50 WHERE id = 1", &[])?;
    db.execute("UPDATE accounts SET balance = balance + 50 WHERE id = 2", &[])?;
    db.execute("COMMIT", &[])?;
    let qr = db.execute("SELECT balance FROM accounts ORDER BY id", &[])?;
    let b0: i64 = qr.rows.first().and_then(|r| r.get(0)).unwrap_or_default();
    let b1: i64 = qr.rows.get(1).and_then(|r| r.get(0)).unwrap_or_default();
    json(200, &ApiResponse { ok: true, data: Some(vec![b0, b1]), error: None })
}

fn txn_rollback() -> Result<Response> {
    let db = Connection::open_default()?;
    db.execute("DELETE FROM accounts", &[])?;
    db.execute("INSERT INTO accounts (balance) VALUES (100), (200)", &[])?;
    db.execute("BEGIN", &[])?;
    db.execute("UPDATE accounts SET balance = balance - 50 WHERE id = 1", &[])?;
    db.execute("UPDATE accounts SET balance = balance + 50 WHERE id = 2", &[])?;
    db.execute("ROLLBACK", &[])?;
    let qr = db.execute("SELECT balance FROM accounts ORDER BY id", &[])?;
    let b0: i64 = qr.rows.first().and_then(|r| r.get(0)).unwrap_or_default();
    let b1: i64 = qr.rows.get(1).and_then(|r| r.get(0)).unwrap_or_default();
    json(200, &ApiResponse { ok: true, data: Some(vec![b0, b1]), error: None })
}
