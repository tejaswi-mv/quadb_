//Check if a number is prime in Rust
fn is_prime(n: u32) -> bool {
  if n <= 1 {
    return false;
  }
  for i in 2..=(n as f64).sqrt().round() as u32 {
    if n % i == 0 {
      return false;
    }
  }
  true
}
