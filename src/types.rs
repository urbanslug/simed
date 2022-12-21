#[derive(Debug)]
pub struct Config {
    pub fasta: Option<String>,         // input fasta file
    pub n: Option<usize>,              // width of the (E)DT
    pub g: bool,                       // generate a linear genome
    pub i: bool,                       // output degenerate string
    pub d: f64,                        // percentage of degeneracy
    pub s: usize,                      // number of variants in a degenerate segment
    pub l: usize,                      // max length of a variant
    pub max_degenerate: Option<usize>, // max number of degenerate letters
}

pub const NUCLEOTIDE_COUNT: usize = 4;
