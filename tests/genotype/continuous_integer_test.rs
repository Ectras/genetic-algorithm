#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{ContinuousGenotype, Genotype, IncrementalGenotype};

#[test]
fn general_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(1..10)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![5, 4, 9, 5, 9, 9, 6, 5, 4, 8],
    );

    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![5, 4, 9, 5, 9, 9, 1, 5, 4, 8]
    );

    assert_eq!(genotype.crossover_indexes(), (0..10).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..10).collect::<Vec<_>>());
}

#[test]
fn general_neighbour() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(1..10)
        .with_allele_neighbour_range(-1..1)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![5, 4, 9, 5, 9, 9, 6, 5, 4, 8],
    );

    genotype.mutate_chromosome_neighbour(&mut chromosome, None, &mut rng);
    genotype.mutate_chromosome_neighbour(&mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![5, 4, 9, 5, 9, 9, 5, 5, 4, 8],
    );

    assert_eq!(genotype.crossover_indexes(), (0..10).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..10).collect::<Vec<_>>());
}

#[test]
fn neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(1)
        .with_allele_range(1..10)
        .with_allele_neighbour_range(-1..1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![5]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(2u32));
    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None)),
        vec![vec![4], vec![6]],
    );

    //FIXME: scale doesn't work for generic
    // println!(
    //     "{:?}",
    //     inspect::population(&genotype.neighbouring_population(&chromosome, Some(0.5)))
    // );
    // assert!(relative_population_eq(
    //     inspect::population(&genotype.neighbouring_population(&chromosome, Some(0.5))),
    //     vec![vec![0.39732498], vec![0.497325]],
    //     0.001,
    // ));
}

#[test]
fn neighbouring_population_2() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(1..10)
        .with_allele_neighbour_range(-1..1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![5, 4],);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None)),
        vec![vec![4, 4], vec![6, 4], vec![5, 3], vec![5, 5],]
    );
}

#[test]
fn neighbouring_population_3() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(1..10)
        .with_allele_neighbour_range(-1..1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![5, 4, 9],);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None)),
        vec![
            vec![4, 4, 9],
            vec![6, 4, 9],
            vec![5, 3, 9],
            vec![5, 5, 9],
            vec![5, 4, 8],
            vec![5, 4, 10],
        ]
    );
}

#[test]
fn neighbouring_population_3_one_sided() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(1..10)
        .with_allele_neighbour_range(0..1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![5, 4, 9],);

    // size makes error as it counts 0.0 twice, this is fine
    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));

    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None)),
        vec![vec![6, 4, 9], vec![5, 5, 9], vec![5, 4, 10],]
    );
}
