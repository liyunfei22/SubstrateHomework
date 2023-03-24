pub fn summary(vec: &[u32]) -> Option<u32> {
  let mut sum: u32 = 0;

  for i in vec.iter() {
      if sum.checked_add(*i) == None {
          return None;
      } else {
          sum += *i;
      }
  }
  Some(sum)
}