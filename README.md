# A timestamp object

## Features

* **serde** Serialization/deserialization with
  [serde](https://crates.io/crates/serde)

* **chrono** conversion between [chrono](https://crates.io/crates/chrono)
  types, also required to parse/deserialize strings

* **sqlx** Encoding/Decoding for [sqlx](https://crates.io/crates/sqlx): As
  integer (nanoseconds) for Sqlite, as TIMESTAMP/TIMESTAMPTZ for PostgreSQL

* **as-float-secs** a legacy feature: the default de/serialization and string
  parsing is to/from float seconds (including integers)
