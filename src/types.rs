#[derive(Debug)]
pub struct Config {
    pub n: usize,              // width of the (E) DT
    pub d: usize,              // percentage of degeneracy
    pub s: usize,              // number of variants in a degenerate segment
    pub l: usize,              // max length of a variant
    pub max_degenerate: usize, // max number of degenerate letters
}

pub const NUCLEOTIDE_COUNT: usize = 4;
