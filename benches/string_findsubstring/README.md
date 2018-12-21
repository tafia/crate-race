# string_findsubstring
Search for the position of a string inside of a string.

* **Baseline**: Searching "abc123" inside of "1abc123"
* **Big_Pattern**: Search for a 50kb string inside of "1" + the 50kb string. Chars for this string were random.
* **Monotonous**: 50kb string of all 1s. Find "abc123" at the end.
* **Almost**: 50kb string repeating "abc12" without the ending "3". Find "abc123" at the end of the string.

| | baseline | monotonous | almost | big_pattern |
| --- | --- | --- | --- | --- |
| **[jetscii](https://crates.io/crates/jetscii)** | 0.021 | 9.979 | 38.638 | *2.422* |
| **[subslice](https://crates.io/crates/subslice)** | 0.032 | *0.568* | 31.214 | 30.351 |
| **[twoway](https://crates.io/crates/twoway)** | 0.026 | 5.283 | *30.016* | 108.622 |
| **[galil_seiferas](https://crates.io/crates/galil_seiferas)** | *0.017* | 62.283 | 62.251 | 117.335 |

Speed units are in microseconds per iteration. Less is better.

## Related Functions

* [string_findbyte](../string_findbyte)

## Crate versions

    jetscii = "0.4.3"    # A tiny library to efficiently search strings and byte slices for sets of ASCII characters or bytes.
    subslice = "0.2.0"          # Fast subslice search
    twoway = "0.2.0"        # Fast substring search for strings and byte strings. Optional SSE4.2 acceleration (if detected at runtim…
    galil-seiferas = "0.1.5"    # General string search in constant space, linear time, for nonorderable alphabets.

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`