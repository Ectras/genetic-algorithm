#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverRange};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};

#[test]
fn population_even() {
    let genotype = BinaryGenotype::builder().with_gene_size(6).build().unwrap();

    let population = build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = CrossoverRange(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, true, false, false, false],
            vec![false, false, true, true, true],
            vec![true, true, false, false, false],
            vec![false, false, true, true, true],
        ]
    )
}

#[test]
fn allow_unique_genotype() {
    assert_eq!(CrossoverRange(false).allow_unique_genotype(), false);
    assert_eq!(CrossoverRange(true).allow_unique_genotype(), false);
}
