Feature: SQL Operations
  As a business domain component
  I want to interact with a sqlite database
  So that I can persist and retrieve business data

  Background:
    Given a sqlite database is available
    And the database is empty

  Scenario: Execute a simple query
    Given I have a table "users" with columns "id SERIAL PRIMARY KEY, name VARCHAR(255)"
    When I query "SELECT * FROM users"
    Then the query should succeed
    And the result should have 0 rows

  Scenario: Insert and retrieve data
    Given I have a table "users" with columns "id SERIAL PRIMARY KEY, name VARCHAR(255), email VARCHAR(255)"
    When I execute "INSERT INTO users (name, email) VALUES ($1, $2)" with params ["John Doe", "john@example.com"]
    Then the execution should succeed
    And 1 row should be affected
    When I query "SELECT name, email FROM users WHERE name = $1" with params ["John Doe"]
    Then the query should succeed
    And the result should have 1 row
    And row 0 column "name" should be "John Doe"
    And row 0 column "email" should be "john@example.com"

  Scenario: Handle connection failures gracefully
    Given the database connection is unavailable
    When I query "SELECT 1"
    Then the query should fail with error "connection-failed"

  Scenario: Handle invalid SQL gracefully
    Given a sqlite database is available
    When I query "INVALID SQL STATEMENT"
    Then the query should fail with error "query-failed"

  Scenario: Transaction management
    Given I have a table "accounts" with columns "id SERIAL PRIMARY KEY, balance INTEGER"
    And I execute "INSERT INTO accounts (balance) VALUES (100), (200)" with params []
    When I begin a transaction
    And I execute "UPDATE accounts SET balance = balance - 50 WHERE id = 1" in transaction
    And I execute "UPDATE accounts SET balance = balance + 50 WHERE id = 2" in transaction
    And I commit the transaction
    Then the transaction should succeed
    When I query "SELECT balance FROM accounts ORDER BY id"
    Then row 0 column "balance" should be 50
    And row 1 column "balance" should be 250

  Scenario: Transaction rollback
    Given I have a table "accounts" with columns "id SERIAL PRIMARY KEY, balance INTEGER"
    And I execute "INSERT INTO accounts (balance) VALUES (100), (200)" with params []
    When I begin a transaction
    And I execute "UPDATE accounts SET balance = balance - 50 WHERE id = 1" in transaction
    And I execute "UPDATE accounts SET balance = balance + 50 WHERE id = 2" in transaction
    And I rollback the transaction
    Then the transaction should succeed
    When I query "SELECT balance FROM accounts ORDER BY id"
    Then row 0 column "balance" should be 100
    And row 1 column "balance" should be 200
