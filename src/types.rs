#[derive(Debug)]
pub struct Config {
    pub n: usize,
    pub d: usize, // percentage of degeneracy
    pub s: usize, // number of variants in a degenerate segment
    pub l: usize, // max number of variants in a degenerate segment
    pub max_degenerate: usize,
}

pub const NUCLEOTIDE_COUNT: usize = 4;
