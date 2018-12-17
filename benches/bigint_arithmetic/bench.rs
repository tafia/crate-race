//!Multiplying numbers larger than what's possible with a normal 64 bit processor.
//!
//!* **Baseline**: Multiply 1 * 1
//!* **Fact50**: Take the factorial of 50. (Gets up to 256 bits)
//!* **Fact95**: Take the factorial of 95. (Gets up to 512 bits)
//!
//!Note: We cannot include GMP or unstable crates. GMP can't be included because it is not crossplatform (not available in Windows).

#[macro_use]
extern crate bencher;

static BASELINE: &str = "1";
static FACT50: &str = "30414093201713378043612608166064768844377641568960512000000000000";
static FACT95: &str = "10329978488239059262599702099394727095397746340117372869212250571234293987594703124871765375385424468563282236864226607350415360000000000000000000000";

mod _num_bigint;
mod _uint;
mod _numext_fixed_uint;

benchmark_group!(baseline, _num_bigint::baseline, _uint::baseline, _numext_fixed_uint::baseline);
benchmark_group!(fact50, _num_bigint::fact50, _uint::fact50, _numext_fixed_uint::fact50);
benchmark_group!(fact95, _num_bigint::fact95, _uint::fact95, _numext_fixed_uint::fact95);

benchmark_main!(baseline, fact50, fact95);
