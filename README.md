# Timestamp/monotonic objects

## Features

* **serde** Serialization/deserialization with
  [serde](https://crates.io/crates/serde)

* **chrono** conversion between [chrono](https://crates.io/crates/chrono)
  types, also required to parse/deserialize strings from human-readable times

* **sqlx** encoding/decoding for [sqlx](https://crates.io/crates/sqlx)

* **as-float-secs** a legacy feature: the default Timestamp de/serialization
  and string parsing is to/from float seconds (including integers)

## sqlx encoding/decoding

### Timestamp

* Sqlite: INTEGER (nanoseconds)

* PostgreSQL: TIMESTAMPTZ/TIMESTAMP

### Monotonic

* Sqlite: INTEGER (nanoseconds)

* PostgreSQL: BIGINT (nanoseconds)
