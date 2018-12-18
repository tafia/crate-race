use bencher::Bencher;

use algos::pattern;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(pattern::horspool(super::BASELINE1, super::BASELINE), Some(1));
  });
}

pub fn big_pattern(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(pattern::horspool(super::BIG_SEARCH, super::BIG_PATTERN), Some(1));
  });
}

pub fn almost(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(pattern::horspool(super::ALMOST, super::BASELINE), Some(50_000));
  });
}

pub fn monotonous(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(pattern::horspool(super::MONOTONOUS, super::BASELINE), Some(50_000));
  });
}
