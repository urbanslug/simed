use coitrees::{COITree, IntervalNode};
use eds::DT;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashSet;

#[derive(Debug)]
struct Config {
    pub n: usize,
    pub d: usize, // percentage of degeneracy
    pub s: usize, // number of variants in a degenerate segment
    pub l: usize, // max number of variants in a degenerate segment
    pub max_degenerate: usize,
}

type Segment = IntervalNode<(), usize>;

fn generate_genome(genome_length: usize) -> String {
    // generate genome with a uniform distribution of bases

    let between: Uniform<u8> = Uniform::from(0..4);
    let mut rng = rand::thread_rng();

    (0..genome_length)
        .map(|_| match between.sample(&mut rng) {
            0 => 'A',
            1 => 'T',
            2 => 'C',
            3 => 'G',
            _ => panic!("Unexpected char"),
        })
        .collect()
}

fn generate_random_base() -> char {
    let between: Uniform<u8> = Uniform::from(0..4);
    let mut rng = rand::thread_rng();

    match between.sample(&mut rng) {
        0 => 'A',
        1 => 'T',
        2 => 'C',
        3 => 'G',
        _ => panic!("Unexpected char"),
    }
}

fn mutate(matrix: &mut Vec<Vec<u8>>, config: &Config) {
    let n = config.n;
    let mut selected_loci = HashSet::<usize>::new();
    let locus_universe: Uniform<usize> = Uniform::from(0..n);
    let variants_universe: Uniform<usize> = Uniform::from(2..config.s + 1);
    let variants_length_universe: Uniform<usize> = Uniform::from(1..config.l + 1);

    let mut rng = rand::thread_rng();

    let mut counter = 0;

    let mut covered: HashSet<usize> = HashSet::with_capacity(config.n);

    eprintln!("locus\tl\ts");
    eprintln!("------------------");

    let mut size = config.n;

    let buffer = 0; // artificial buffer between degenerate letters

    loop {
        if counter >= config.max_degenerate {
            break;
        }

        let locus = locus_universe.sample(&mut rng);
        match selected_loci.insert(locus) {
            false => continue,
            _ => {}
        }

        let s = variants_universe.sample(&mut rng);
        let l = variants_length_universe.sample(&mut rng);

        let degenerate_start = locus;
        let degenerate_stop = locus + l;

        if degenerate_stop >= config.n {
            continue;
        }

        let set1: HashSet<usize> = HashSet::from_iter(degenerate_start..degenerate_stop + buffer);

        if covered.intersection(&set1).count() > 0 {
            // eprintln!("skipping overlap {:?} {:?}", covered, set1);
            // eprintln!("skipping overlap {} {}", degenerate_start, degenerate_stop);
            continue;
        }

        let x =
            (degenerate_start..degenerate_stop).fold(true, |acc, pos| acc && covered.insert(pos));

        if !x {
            panic!("Failed interval check ({degenerate_start}, {degenerate_stop})");
        }

        for i in degenerate_start..degenerate_stop {
            // max entropy
            let mut s_prime = 0;
            loop {
                if s_prime >= s {
                    break;
                }

                let base = generate_random_base() as u8;

                if s < 4 && matrix[i].iter().any(|b| *b == base) {
                    continue;
                }
                matrix[i].push(base as u8);
                size += 1;
                s_prime += 1;
            }
        }

        // eprintln!("{}\t{}\t{}\t{}", degenerate_start, degenerate_stop, l, s);
        counter += 1;
    }

    let dt = DT {
        data: matrix.clone(),
        z: config.s,
    };

    // eprintln!("{}", dt);
}

fn percent(d: usize, n: usize) -> usize {
    ((d as f64 / 100_f64) * n as f64).floor() as usize
}

fn main() {
    let n: usize = 1_000_000;
    let d: usize = 10;
    let x = percent(d, n);

    let config = Config {
        n,
        d,
        s: 3,
        l: 5,
        max_degenerate: x,
    };

    eprintln!("Stats");
    eprintln!("max degenerate loci: {}", config.max_degenerate * config.l);

    if (config.max_degenerate * config.l) >= percent(25, config.n) {
        eprintln!("Warning: too much variation. More than 25% variation");
    }

    eprintln!("{:?}", config);

    let genome = generate_genome(config.n);
    // eprintln!("{}", genome);

    let mut matrix: Vec<Vec<u8>> = genome.chars().map(|c| vec![c as u8]).collect();
    mutate(&mut matrix, &config);

    eprintln!("Stats");
    eprintln!("-----");
}
