# chalmersproject/api

_The API backend for the Chalmers Project._

## Usage

Try the currently deployed API playground at: https://api.chalmersproject.com

## Development

> You'll need the latest versions of
> [the Rust language toolchain](https://rustup.rs),
> [Docker](https://docs.docker.com/get-docker/), and
> [Docker Compose](https://docs.docker.com/compose/install/).

```bash
# If you don't have a .env, scaffold one:
cp .env.example .env

# Configure your .env:
vi .env

# Run prerequisite services, like Postgres:
docker-compose up -d

# Apply database migrations:
cargo run -- migrate

# Start server:
cargo run -- serve

# Try the GraphQL playground:
open http://localhost:8080

# ...

# Shut down prerequisite services (cleanup):
docker-compose down
```
