pub fn percent(d: usize, n: usize) -> usize {
    ((d as f64 / 100_f64) * n as f64).floor() as usize
}
