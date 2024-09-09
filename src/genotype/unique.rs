use super::builder::{Builder, TryFromBuilderError};
use super::{Allele, Genotype, IncrementalGenotype, PermutableGenotype};
use crate::chromosome::{ChromosomeManager, LegacyChromosome};
use factorial::Factorial;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub type DefaultAllele = usize;

/// Genes are a vector of unique values, taken from the allele_list using clone(), each value occurs
/// exactly once. The genes_size is derived to be the same as allele_list length. On random
/// initialization, the allele_list are shuffled to form the genes. Each pair of genes has an equal
/// probability of mutating. If a pair of genes mutates, the values are switched, ensuring the list
/// of alleles remains unique. Defaults to usize as item.
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, UniqueGenotype};
///
/// let genotype = UniqueGenotype::builder()
///     .with_allele_list((0..100).collect())
///     .build()
///     .unwrap();
/// ```
///
/// # Example (struct)
/// ```
/// use genetic_algorithm::genotype::{Allele, Genotype, UniqueGenotype};
///
/// #[derive(Clone, Copy, PartialEq, Debug)]
/// struct Item(pub u16, pub u16);
/// impl Allele for Item {}
///
/// let genotype = UniqueGenotype::builder()
///     .with_allele_list(vec![
///         Item(23, 505),
///         Item(26, 352),
///         Item(20, 458),
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Unique<T: Allele = DefaultAllele> {
    pub genes_size: usize,
    pub allele_list: Vec<T>,
    gene_index_sampler: Uniform<usize>,
    pub seed_genes_list: Vec<Vec<T>>,
    pub chromosome_recycling: bool,
    pub chromosome_bin: Vec<LegacyChromosome<Self>>,
    pub best_genes: Vec<T>,
}

impl<T: Allele> TryFrom<Builder<Self>> for Unique<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.allele_list.is_none() {
            Err(TryFromBuilderError("UniqueGenotype requires allele_list"))
        } else if builder.allele_list.as_ref().map(|o| o.is_empty()).unwrap() {
            Err(TryFromBuilderError(
                "UniqueGenotype requires non-empty allele_list",
            ))
        } else {
            let allele_list = builder.allele_list.unwrap();
            let genes_size = allele_list.len();
            Ok(Self {
                genes_size,
                allele_list: allele_list.clone(),
                gene_index_sampler: Uniform::from(0..allele_list.len()),
                seed_genes_list: builder.seed_genes_list,
                chromosome_recycling: builder.chromosome_recycling,
                chromosome_bin: vec![],
                best_genes: allele_list.clone(),
            })
        }
    }
}

impl<T: Allele> Genotype for Unique<T> {
    type Allele = T;
    type Genes = Vec<Self::Allele>;

    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn store_best_genes(&mut self, chromosome: &LegacyChromosome<Self>) {
        self.best_genes.clone_from(&chromosome.genes);
    }
    fn get_best_genes(&self) -> &Self::Genes {
        &self.best_genes
    }

    fn mutate_chromosome_genes<R: Rng>(
        &mut self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut LegacyChromosome<Self>,
        _scale_index: Option<usize>,
        rng: &mut R,
    ) {
        if allow_duplicates {
            for _ in 0..number_of_mutations {
                let index1 = self.gene_index_sampler.sample(rng);
                let index2 = self.gene_index_sampler.sample(rng);
                chromosome.genes.swap(index1, index2);
            }
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size,
                (number_of_mutations * 2).min(self.genes_size),
            )
            .iter()
            .tuples()
            .for_each(|(index1, index2)| chromosome.genes.swap(index1, index2));
        }
        chromosome.taint_fitness_score();
    }
    fn crossover_chromosome_genes<R: Rng>(
        &mut self,
        _number_of_crossovers: usize,
        _allow_duplicates: bool,
        _father: &mut LegacyChromosome<Self>,
        _mother: &mut LegacyChromosome<Self>,
        _rng: &mut R,
    ) {
        panic!("UniqueGenotype does not support gene crossover")
    }
    fn crossover_chromosome_points<R: Rng>(
        &mut self,
        _number_of_crossovers: usize,
        _allow_duplicates: bool,
        _father: &mut LegacyChromosome<Self>,
        _mother: &mut LegacyChromosome<Self>,
        _rng: &mut R,
    ) {
        panic!("UniqueGenotype does not support point crossover")
    }

    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Self::Genes>) {
        self.seed_genes_list = seed_genes_list;
    }
    fn seed_genes_list(&self) -> &Vec<Self::Genes> {
        &self.seed_genes_list
    }
    fn max_scale_index(&self) -> Option<usize> {
        None
    }
}

impl<T: Allele> IncrementalGenotype for Unique<T> {
    fn neighbouring_chromosomes<R: Rng>(
        &self,
        chromosome: &LegacyChromosome<Self>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) -> Vec<LegacyChromosome<Self>> {
        (0..self.genes_size())
            .tuple_combinations()
            .map(|(first, second)| {
                let mut new_genes = chromosome.genes.clone();
                new_genes.swap(first, second);
                new_genes
            })
            .map(LegacyChromosome::new)
            .collect::<Vec<_>>()
    }

    fn neighbouring_population_size(&self) -> BigUint {
        let n = BigUint::from(self.genes_size);
        let k = BigUint::from(2usize);

        n.factorial() / (k.factorial() * (n - k).factorial())
    }
}

impl<T: Allele> PermutableGenotype for Unique<T> {
    fn chromosome_permutations_into_iter(
        &self,
    ) -> impl Iterator<Item = LegacyChromosome<Self>> + Send {
        self.allele_list
            .clone()
            .into_iter()
            .permutations(self.genes_size())
            .map(LegacyChromosome::new)
    }

    fn chromosome_permutations_size(&self) -> BigUint {
        BigUint::from(self.genes_size).factorial()
    }
}

impl<T: Allele> ChromosomeManager<Self> for Unique<T> {
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> <Self as Genotype>::Genes {
        if self.seed_genes_list.is_empty() {
            let mut genes = self.allele_list.clone();
            genes.shuffle(rng);
            genes
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn chromosome_constructor_empty(&self) -> LegacyChromosome<Self> {
        LegacyChromosome::new(vec![])
    }
    fn chromosome_is_empty(&self, chromosome: &LegacyChromosome<Self>) -> bool {
        chromosome.genes.is_empty()
    }
    fn chromosome_recycling(&self) -> bool {
        self.chromosome_recycling
    }
    fn chromosome_bin_push(&mut self, chromosome: LegacyChromosome<Self>) {
        self.chromosome_bin.push(chromosome);
    }
    fn chromosome_bin_pop(&mut self) -> Option<LegacyChromosome<Self>> {
        self.chromosome_bin.pop()
    }
}

impl<T: Allele> fmt::Display for Unique<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  allele_list: {:?}", self.allele_list)?;
        writeln!(
            f,
            "  chromosome_permutations_size: {}",
            self.chromosome_permutations_size()
        )?;
        writeln!(
            f,
            "  neighbouring_population_size: {}",
            self.neighbouring_population_size()
        )?;
        writeln!(f, "  seed_genes_list: {:?}", self.seed_genes_list)
    }
}
