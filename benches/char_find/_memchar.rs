use bencher::Bencher;

pub fn baseline(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(memchr::memchr(b'/', super:BASELINE), Some(0));
  });
}

pub fn big_str_first(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(memchr::memchr(b'/', super:BIG_STR_FIRST), Some(0));
  });
}

pub fn big_str_last(b: &mut Bencher) {
  b.iter(|| {
    assert_eq!(memchr::memchr(b'/', super:BIG_STR_LAST), Some(50_000));
  });
}
