# json_parse
Parsing JSONs, and retrieving values at specific rows/columns.

* **Baseline**: Simple json with just one entry.
* **Serial**: Opposite of nested. 100 entries listed in a "flat" format, one by one, without any tree structure.
* **Nested**: 100 nested entries, each inside of the other.

| | baseline | serial | nested |
| --- | --- | --- | --- |
| **[json](https://crates.io/crates/json)** | *0.188* | *7.727* | *12.52* |
| **[serde_json](https://crates.io/crates/serde_json)** | 0.292 | 22.373 | 25.617 |
| **[json5](https://crates.io/crates/json5)** | 2.442 | 148.581 | 152.916 |

Speed units are in microseconds per iteration. Less is better.

## Related Functions

* [json_build](../json_build)

## Crate versions

    json = "0.11.13"                          # JSON implementation in Rust
    serde_json = "1.0.33"                      # A JSON serialization file format
    json5 = "0.2.2"            # A Rust JSON5 serializer and deserializer which speaks Serde.

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`