use bencher::Bencher;

use numext_fixed_uint::{U256, U512};

pub fn baseline(b: &mut Bencher) {
  
  b.iter(|| {
    let num_1 = U512::one();
    let mut num_sum = U512::one();
    num_sum += &num_1;
    assert_eq!(U512::from(2u8), num_sum);
  });
}


pub fn fact50(b: &mut Bencher) {
  b.iter(|| {
    let num_1 = U256::one();
    let mut num_sum = U256::one();
    let mut fact = U256::one();
    for _ in 1..50 {
      num_sum += &num_1;
      fact *= &num_sum;
    }
    let ten = U256::from(10u8);
    assert_eq!(U256::from(0u8), &fact % &ten); //Most right digit
    for _ in 0..64 {    //Remove all other digits
      fact /= &ten;
    }
    assert_eq!(U256::from(3u8), fact); //Most left digit
  });
}


pub fn fact95(b: &mut Bencher) {
  b.iter(|| {
    let num_1 = U512::one();
    let mut num_sum = U512::one();
    let mut fact = U512::one();
    for _ in 1..95 {
      num_sum += &num_1;
      fact *= &num_sum;
    }
    let ten = U512::from(10u8);
    assert_eq!(U512::from(0u8), &fact % &ten); //Most right digit
    for _ in 0..148 {    //Remove all other digits
      fact /= &ten;
    }
    assert_eq!(U512::from(1u8), fact); //Most left digit
  });
}
