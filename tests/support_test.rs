mod support;

use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::genotype::{BinaryGenotype, ListGenotype, RangeGenotype};
use genetic_algorithm::population::Population;

#[cfg(test)]
use crate::support::*;

#[test]
fn chromosome_binary() {
    let chromosome: Chromosome<BinaryGenotype> = build::chromosome(vec![true, false, true, false]);
    println!("{:#?}", chromosome);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![true, false, true, false]
    );
}

#[test]
fn chromosome_list() {
    let chromosome: Chromosome<ListGenotype<u8>> = build::chromosome(vec![3, 4, 5, 6]);
    println!("{:#?}", chromosome);
    assert_eq!(inspect::chromosome(&chromosome), vec![3, 4, 5, 6]);
}

#[test]
fn chromosome_range() {
    let chromosome: Chromosome<RangeGenotype<f32>> = build::chromosome(vec![0.1, 0.2, 0.3]);
    println!("{:#?}", chromosome);
    assert_eq!(inspect::chromosome(&chromosome), vec![0.1, 0.2, 0.3]);
}

#[test]
fn population_binary() {
    let population: Population<BinaryGenotype> = build::population(vec![
        vec![true, true, true],
        vec![true, true, false],
        vec![true, false, false],
        vec![false, false, false],
    ]);
    println!("{:#?}", population);
    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, false],
            vec![false, false, false],
        ]
    );
}
