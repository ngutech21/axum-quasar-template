[![Rust](https://github.com/ngutech21/axum-quasar/actions/workflows/run-tests.yml/badge.svg?branch=master)](https://github.com/ngutech21/axum-quasar/actions/workflows/run-tests.yml)
[![Frontend](https://github.com/ngutech21/axum-quasar/actions/workflows/build-frontend.yml/badge.svg?branch=master)](https://github.com/ngutech21/axum-quasar/actions/workflows/build-frontend.yml)

# required tools

- docker-compose
- cargo sqlx-cli
- nodejs
- yarn
  
## setup

### build frontend
```
cd frontend
yarn global add @quasar/cli
yarn
yarn quasar build
```
### run backend
```
sqlx migrate run
cargo run
```
open <http://127.0.0.1:8080>
