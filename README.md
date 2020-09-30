# @cardbox/backend

## Directories and crates

- `db` — database schema, can be reused in different crates
- `core` — main crate with business-logic of the cardbox
- `api-private` — crate with actix-web http2 routes, used only inside private network
- `api-internal` — crate with http server, used only by cardbox frontend
- `api-admin` — crate with http server, used only by cardbox admin frontend
- `api-public` — crate with http server, used from outside. Ex.: accesso calls it when profile updates
