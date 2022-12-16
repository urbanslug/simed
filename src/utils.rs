pub fn percent(d: f64, n: usize) -> usize {
    ((d / 100_f64) * n as f64).floor() as usize
}
