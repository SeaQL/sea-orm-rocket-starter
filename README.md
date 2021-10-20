# SeaORM Rocket Starter

This project intends to be a project template for you to clone and start writing a web service on the Rocket + SeaORM stack.

It exposes a REST API performing OLTP on a relational database (MySQL or Postgres).

It has unit tests (on SQLite) built-in as well as GitHub Actions for continuous integration.

Probably, some day, we will name this project template "Rust on Rocket" 🚀!


## Requirements
Postgres >= 13

## To run the app:
`cargo run`
Use your API client to access the resources: `http://127.0.0.1:8000/[resource_name]/[id]`
## To run the test:
`cargo test`
