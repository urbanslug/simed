use rand::distributions::{Distribution, Uniform};
use std::collections::{HashMap, HashSet};

mod cli;
mod io;
mod types;
mod utils;

fn generate_random_base(with_epsilons: bool) -> char {
    let max: u8 = if with_epsilons { 5 } else { 4 };
    let between: Uniform<u8> = Uniform::from(0..max);
    let mut rng = rand::thread_rng();

    match between.sample(&mut rng) {
        0 => 'A',
        1 => 'T',
        2 => 'C',
        3 => 'G',
        4 => '*',
        _ => panic!("Unexpected char"),
    }
}

fn mutate(
    matrix: &mut Vec<Vec<u8>>,
    config: &types::Config,
    genome_length: usize,
) -> HashMap<usize, (usize, usize)> {
    let mut selected_loci = HashSet::<usize>::new();
    let locus_universe: Uniform<usize> = Uniform::from(0..genome_length);
    let variants_universe: Uniform<usize> = Uniform::from(1..config.s);
    let variants_length_universe: Uniform<usize> = Uniform::from(1..config.l + 1);

    let mut rng = rand::thread_rng();

    let mut counter = 0;

    let mut degenerate_regions =
        HashMap::<usize, (usize, usize)>::with_capacity(config.max_degenerate.unwrap());
    let mut covered: HashSet<usize> = HashSet::with_capacity(genome_length);

    let mut size = genome_length;

    let buffer = 0; // artificial buffer between degenerate letters

    loop {
        if counter >= config.max_degenerate.unwrap() {
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

        if degenerate_stop >= genome_length {
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

        degenerate_regions.insert(degenerate_start, (l, s));

        for i in degenerate_start..degenerate_stop {
            // max entropy
            let mut s_prime = 0;
            loop {
                if s_prime >= s {
                    break;
                }

                let base = generate_random_base(!config.i) as u8;

                if s < types::NUCLEOTIDE_COUNT && matrix[i].iter().any(|b| *b == base) {
                    continue;
                }
                matrix[i].push(base as u8);
                if base != b'*' {
                    size += 1;
                }
                s_prime += 1;
            }
        }

        // eprintln!("{}\t{}\t{}\t{}", degenerate_start, degenerate_stop, l, s);
        counter += 1;
    }

    degenerate_regions
}

fn write_eds(dt: &Vec<Vec<u8>>, lookup: &HashMap<usize, (usize, usize)>, genome_length: usize) {
    let mut pos = 0;

    loop {
        if pos >= genome_length {
            break;
        }

        match lookup.get(&pos).copied() {
            None => {
                let col: String = dt[pos].iter().take(1).map(|c| *c as char).collect();
                print!("{}", col);
                pos += 1;
            }
            Some((length, depth)) => {
                print!("{{");
                for s in 0..=depth {
                    for l in pos..pos + length {
                        let c = dt[l][s] as char;
                        if c != '*' {
                            print!("{c}");
                        }
                    }
                    if s < depth {
                        print!(",");
                    }
                }

                print!("}}");
                pos += length;
            }
        }
    }
}

fn main() {
    let mut config = cli::start();

    let genome = match &config.fasta {
        Some(f) => io::read_sequence(&f),
        _ => io::generate_sequence(config.n.unwrap()),
    };

    let genome_length = genome.len();

    let x = utils::percent(config.d, genome_length);
    config.max_degenerate = Some(x);

    eprintln!(
        "Config:\n\
         {0:indent_two$}Genome length/width: {1}\n\
         {0:indent_two$}Degeneracy: {2}%\n\
         {0:indent_two$}Inelastic: {3}\n\
         {0:indent_two$}Max variants in a degenerate segment: {4}\n\
         {0:indent_two$}Max length of a variant: {5}\n\
         {0:indent_two$}Max number of degenerate letters in (E)DT: {6}",
        "",
        genome_length,
        config.d,
        config.i,
        config.s,
        config.l,
        config.max_degenerate.unwrap(),
        indent_two = 2,
    );

    if (config.max_degenerate.unwrap() * config.l) >= utils::percent(25.0, genome_length) {
        eprintln!("Warning: too much variation. More than 25% variation");
    }

    let mut matrix: Vec<Vec<u8>> = genome.chars().map(|c| vec![c as u8]).collect();
    let region_map = mutate(&mut matrix, &config, genome_length);

    // output eds format
    write_eds(&matrix, &region_map, genome_length);
}
