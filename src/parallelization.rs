#[cfg(test)]
mod tests {
  use std::time::SystemTime;
  use rayon::iter::*;

  #[test]
  fn parallelization_test() {
    let mut numbers = Vec::new();
    for i in 1..1000000 {
      numbers.push(i);
    }

    let t0 = SystemTime::now();
    let seq_sum = *numbers.iter().max().unwrap();
    let t1 = SystemTime::now();
    let par_sum = numbers.par_iter().map(|&x| x).reduce(|| 0, |x, y| if x > y {x} else {y});
    let t2 = SystemTime::now();

    assert_eq!(par_sum, seq_sum);

    println!("Sequential Sum Time: {} µs\r\nParallel Sum Time: {} µs",
      t1.duration_since(t0).unwrap().as_micros(),
      t2.duration_since(t1).unwrap().as_micros());
  }
}
