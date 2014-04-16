
fn main() {
  let mut total : i32 = 0;
  let mut i = 1;
  while i < 1000 {
    if i % 3 == 0 || i % 5 == 0 {
      total += i;
    }
    i += 1;
  }

  println!("{:i}", total);
}