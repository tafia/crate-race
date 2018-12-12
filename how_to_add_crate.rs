#How To Add a Crate

1. Make a file `crate-race/benches/*/_[crate].rs` with benchmarks specific to that crate.
2. Update `crate-race/benches/*/bench.rs`:
  a. add `mod _[crate];`
  b. Update any listed `benchmark_group!` and `benchmark_main!` to include `[crate]::function`
3. Update `crate-race/Cargo.toml`, with `[crate] = "*"` close to other crates of the same bench.
4. Update `crate_list.csv`, with [crate],0,[bench1],[bench2],[bench3],etc. Order the crate alphabetically.
