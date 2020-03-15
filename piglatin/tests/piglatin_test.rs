use piglatin::*;

#[test]
fn test_back() {
  let word = "back";
  assert_eq!("ack-bay", translate(&word))
}

#[test]
fn test_first() {
  let word = "first";
  assert_eq!("irst-fay", translate(&word))
}

#[test]
fn test_apple() {
  let word = "apple";
  assert_eq!("apple-hay", translate(&word))
}
