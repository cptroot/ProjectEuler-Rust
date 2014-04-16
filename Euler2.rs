
fn main() {
  let mut total = 0;
  let (mut a, mut b) = (1, 1);
  let mut temp;

  while b < 4000000 {
    if b % 2 == 0 {
      total += b;
    }

    temp = a + b;
    a = b;
    b = temp;
  }

  println!("{:i}", total);
}