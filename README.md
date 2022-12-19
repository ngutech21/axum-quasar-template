# required tools

- docker-compose
- cargo sqlx-cli
- nodejs
- yarn
  
## setup

### build frontend

cd frontend
yarn global add @quasar/cli
yarn
yarn quasar build

### run backend

sqlx migrate run
cargo run

open <http://127.0.0.1:8080>
