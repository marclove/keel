# Keel E2E App

A minimal Spin HTTP app that exercises SQLite operations used by Keel components.

Endpoints:
- POST `/setup` – creates tables and clears data
- POST `/users` – body: `{ "name": string, "email": string }`
- GET `/users` – lists users
- POST `/txn/commit` – performs a transfer and commits; returns balances
- POST `/txn/rollback` – performs a transfer and rolls back; returns balances

Local run:
- Build: `just spin-build apps/e2e-keel`
- Run: `just spin-up apps/e2e-keel`
- Example:
  - `curl -X POST localhost:3000/setup`
  - `curl -X POST localhost:3000/users -H 'content-type: application/json' -d '{"name":"Alice","email":"a@example.com"}'`
  - `curl localhost:3000/users`
  - `curl -X POST localhost:3000/txn/commit`
  - `curl -X POST localhost:3000/txn/rollback`

Note: Configure database bindings if required by your Spin runtime; by default the app uses `Connection::open_default()` which expects a `default` SQLite database binding.
