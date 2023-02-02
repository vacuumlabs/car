# Crypto Addresses Relation

Find information about addresses and sort them by service and tags.
It starts with storing data in the PG and querying data using the GIN extension. But we do not know the direction of performance.
If we want to process all data before the query or get relevant information during the query.

Set parm database URL before.

## Service

Identify a kind of service under the blockchain. For example Dex, Web3 game, etc.

## Tag

Tak the address with information like money, sports game, etc.

Runninng

If you want to set up the database using the provided docker compose file:
```bash
export DATABASE_URL="postgres://postgres:postgres@0.0.0.0:5432/addresses"
docker-compose up -d
```

Or if running locally:
```bash
export DATABASE_URL="postgres://postgres:postgres@127.0.0.1:5432/addresses"
```

Then run:
```bash
cargo install sea-orm-cli
sea-orm-cli migrate up
cargo run
```

== Frontend

```bash
cd frontend
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve
```
