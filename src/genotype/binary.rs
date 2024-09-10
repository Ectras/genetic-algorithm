use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, IncrementalGenotype, PermutableGenotype};
use crate::chromosome::{BinaryChromosome, Chromosome, ChromosomeManager, OwnsGenes};
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Standard, Uniform};
use rand::prelude::*;
use std::fmt;

/// Genes are a vector of booleans. On random initialization, each gene has a 50% probability of
/// becoming true or false. Each gene has an equal probability of mutating. If a gene mutates, its
/// value is flipped.
///
/// # Example:
/// ```
/// use genetic_algorithm::genotype::{Genotype, BinaryGenotype};
///
/// let genotype = BinaryGenotype::builder()
///     .with_genes_size(100)
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct Binary {
    pub genes_size: usize,
    gene_index_sampler: Uniform<usize>,
    pub seed_genes_list: Vec<Vec<bool>>,
    pub chromosome_recycling: bool,
    pub chromosome_bin: Vec<BinaryChromosome>,
    pub best_genes: Vec<bool>,
}

impl TryFrom<Builder<Self>> for Binary {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.genes_size.is_none() {
            Err(TryFromBuilderError("BinaryGenotype requires a genes_size"))
        } else {
            let genes_size = builder.genes_size.unwrap();
            Ok(Self {
                genes_size,
                gene_index_sampler: Uniform::from(0..genes_size),
                seed_genes_list: builder.seed_genes_list,
                chromosome_recycling: builder.chromosome_recycling,
                chromosome_bin: vec![],
                best_genes: vec![false; genes_size],
            })
        }
    }
}

impl Genotype for Binary {
    type Allele = bool;
    type Genes = Vec<Self::Allele>;
    type Chromosome = BinaryChromosome;

    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn save_best_genes(&mut self, chromosome: &Self::Chromosome) {
        self.best_genes.clone_from(&chromosome.genes);
    }
    fn load_best_genes(&mut self, chromosome: &mut Self::Chromosome) {
        chromosome.genes.clone_from(&self.best_genes);
    }
    fn best_genes(&self) -> &Self::Genes {
        &self.best_genes
    }

    fn mutate_chromosome_genes<R: Rng>(
        &mut self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Self::Chromosome,
        _scale_index: Option<usize>,
        rng: &mut R,
    ) {
        if allow_duplicates {
            rng.sample_iter(self.gene_index_sampler)
                .take(number_of_mutations)
                .for_each(|index| {
                    chromosome.genes[index] = !chromosome.genes[index];
                });
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size,
                number_of_mutations.min(self.genes_size),
            )
            .iter()
            .for_each(|index| {
                chromosome.genes[index] = !chromosome.genes[index];
            });
        }
        chromosome.taint();
    }

    fn crossover_chromosome_genes<R: Rng>(
        &mut self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Self::Chromosome,
        mother: &mut Self::Chromosome,
        rng: &mut R,
    ) {
        if allow_duplicates {
            rng.sample_iter(self.gene_index_sampler)
                .take(number_of_crossovers)
                .for_each(|index| {
                    std::mem::swap(&mut father.genes[index], &mut mother.genes[index]);
                });
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size(),
                number_of_crossovers.min(self.genes_size()),
            )
            .iter()
            .for_each(|index| {
                std::mem::swap(&mut father.genes[index], &mut mother.genes[index]);
            });
        }
        mother.taint();
        father.taint();
    }
    fn crossover_chromosome_points<R: Rng>(
        &mut self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Self::Chromosome,
        mother: &mut Self::Chromosome,
        rng: &mut R,
    ) {
        if allow_duplicates {
            rng.sample_iter(self.gene_index_sampler)
                .take(number_of_crossovers)
                .for_each(|index| {
                    let mother_back = &mut mother.genes[index..];
                    let father_back = &mut father.genes[index..];
                    father_back.swap_with_slice(mother_back);
                });
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size(),
                number_of_crossovers.min(self.genes_size()),
            )
            .iter()
            .sorted_unstable()
            .chunks(2)
            .into_iter()
            .for_each(|mut chunk| match (chunk.next(), chunk.next()) {
                (Some(start_index), Some(end_index)) => {
                    let mother_back = &mut mother.genes[start_index..end_index];
                    let father_back = &mut father.genes[start_index..end_index];
                    father_back.swap_with_slice(mother_back);
                }
                (Some(start_index), _) => {
                    let mother_back = &mut mother.genes[start_index..];
                    let father_back = &mut father.genes[start_index..];
                    father_back.swap_with_slice(mother_back);
                }
                _ => (),
            });
        }
        mother.taint();
        father.taint();
    }

    fn has_crossover_indexes(&self) -> bool {
        true
    }
    fn has_crossover_points(&self) -> bool {
        true
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

impl IncrementalGenotype for Binary {
    fn neighbouring_chromosomes<R: Rng>(
        &mut self,
        chromosome: &Self::Chromosome,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) -> Vec<Self::Chromosome> {
        (0..self.genes_size)
            .map(|index| {
                let mut new_chromosome = self.chromosome_constructor_from(chromosome);
                new_chromosome.genes[index] = !new_chromosome.genes[index];
                new_chromosome
            })
            .collect::<Vec<_>>()
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(self.genes_size)
    }
}

impl PermutableGenotype for Binary {
    fn chromosome_permutations_into_iter(&self) -> impl Iterator<Item = Self::Chromosome> + Send {
        (0..self.genes_size())
            .map(|_| vec![true, false])
            .multi_cartesian_product()
            .map(BinaryChromosome::new)
    }
    fn chromosome_permutations_size(&self) -> BigUint {
        BigUint::from(2u8).pow(self.genes_size() as u32)
    }
}

impl ChromosomeManager<Self> for Binary {
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<bool> {
        if self.seed_genes_list.is_empty() {
            rng.sample_iter(Standard).take(self.genes_size).collect()
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn chromosome_recycling(&self) -> bool {
        self.chromosome_recycling
    }
    fn chromosome_bin_push(&mut self, chromosome: BinaryChromosome) {
        self.chromosome_bin.push(chromosome);
    }
    fn chromosome_bin_pop(&mut self) -> Option<BinaryChromosome> {
        self.chromosome_bin.pop()
    }
    fn chromosome_constructor<R: Rng>(&mut self, rng: &mut R) -> BinaryChromosome {
        if self.chromosome_recycling() {
            if let Some(mut new_chromosome) = self.chromosome_bin_pop() {
                new_chromosome
                    .genes
                    .clone_from(&self.random_genes_factory(rng));
                new_chromosome.taint();
                new_chromosome
            } else {
                BinaryChromosome::new(self.random_genes_factory(rng))
            }
        } else {
            BinaryChromosome::new(self.random_genes_factory(rng))
        }
    }
    fn chromosome_cloner(&mut self, chromosome: &BinaryChromosome) -> BinaryChromosome {
        if self.chromosome_recycling() {
            if let Some(mut new_chromosome) = self.chromosome_bin_pop() {
                new_chromosome.genes.clone_from(&chromosome.genes);
                new_chromosome.age = chromosome.age;
                new_chromosome.fitness_score = chromosome.fitness_score;
                new_chromosome.reference_id = chromosome.reference_id;
                new_chromosome
            } else {
                chromosome.clone()
            }
        } else {
            chromosome.clone()
        }
    }
    fn chromosome_constructor_from(&mut self, chromosome: &BinaryChromosome) -> BinaryChromosome {
        if self.chromosome_recycling() {
            if let Some(mut new_chromosome) = self.chromosome_bin_pop() {
                new_chromosome.genes.clone_from(&chromosome.genes);
                new_chromosome.taint();
                new_chromosome
            } else {
                chromosome.clone_and_taint()
            }
        } else {
            chromosome.clone_and_taint()
        }
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
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
