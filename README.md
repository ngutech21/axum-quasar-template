# required tools

- docker-compose
- cargo sqlx-cli
- nodejs
- yarn
  
## setup

### build frontend

cd quasar-project
yarn global add @quasar/cli
yarn
yarn quasar build

### run backend

sqlx migrate run
cargo run

<http://127.0.0.1:3000/import_movies>
