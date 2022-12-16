use needletail::parse_fastx_file;
use rand::distributions::{Distribution, Uniform};

pub fn read_sequence(fp: &str) -> String {
    let mut reader = parse_fastx_file(fp)
        .unwrap_or_else(|_| panic!("[simed::read_sequence] invalid fasta path/file {}", fp));

    let seq_record = reader
        .next()
        .expect("[simed::read_sequence] end of iter")
        .expect("[simed::read_sequence] invalid record");

    let name: Vec<u8> = seq_record
        .id()
        .iter()
        .take_while(|x| !(**x).is_ascii_whitespace())
        .cloned()
        .collect();

    let seq = seq_record.seq();
    let num_bases = seq.len();

    eprintln!(
        "{0:two_spaces$}Done processing fasta: \n\
         {0:four_spaces$}Sequence: {name} \n\
         {0:four_spaces$}Number of bases: {bases}.",
        "",
        bases = num_bases,
        name = std::str::from_utf8(&name).unwrap(),
        two_spaces = 2,
        four_spaces = 4
    );

    std::str::from_utf8(&seq).unwrap().to_string()
}

pub fn generate_sequence(genome_length: usize) -> String {
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
