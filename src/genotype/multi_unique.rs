use super::builder::{Builder, TryFromBuilderError};
use super::{EvolveGenotype, Genotype, IncrementalGenotype, PermutableGenotype};
use crate::allele::Allele;
use crate::chromosome::{Chromosome, ChromosomeManager, GenesOwner, MultiUniqueChromosome};
use crate::population::Population;
use factorial::Factorial;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::collections::HashMap;
use std::fmt;

pub type DefaultAllele = usize;

/// Genes are a concatinated vector of sets of unique values, each set taken from its own
/// allele_list using clone(). The genes_size is derived to be the sum of the allele_list
/// lengths. All allele_list have to be of the same type, but can have different values and
/// lengths. On random initialization, the allele_list sets are internally shuffled and
/// concatinated to form the genes, but the order of the sets is always the same. Each unique set
/// has a weighted probability of mutating, depending on its allele_list length. If a set
/// mutates, the values for a pair of genes in the set are switched, ensuring the set remains
/// unique. Duplicate allele values are allowed. Defaults to usize as item.
///
/// # Panics
///
/// Does not support gene crossover, only point crossover is supported. Will panic is gene
/// crossoveris tried, but [EvolveBuilder](crate::strategy::evolve::EvolveBuilder) shouldn't allow
/// this.
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiUniqueGenotype};
///
/// let genotype = MultiUniqueGenotype::builder()
///     .with_allele_lists(vec![
///        (0..=3).collect(),
///        (4..=6).collect(),
///        (7..=9).collect(),
///        (0..=2).collect(),
///     ])
///     .build()
///     .unwrap();
///
/// // chromosome genes example: [1,2,3, 5,4,6, 9,8,7, 1,0]
/// // four unique sets internally shuffled
/// ```
///
/// # Example (struct, the limitation is that the type needs to be the same for all lists)
/// ```
/// use genetic_algorithm::genotype::{Allele, Genotype, MultiUniqueGenotype};
///
/// #[derive(Clone, Copy, PartialEq, Debug)]
/// struct Item(pub u16, pub u16);
/// impl Allele for Item {}
///
/// let genotype = MultiUniqueGenotype::builder()
///     .with_allele_lists(vec![
///       vec![Item(1, 505), Item(2, 352), Item(3, 458)],
///       vec![Item(4, 505), Item(5, 352)],
///       vec![Item(6, 352), Item(7, 458), Item(8, 123)],
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct MultiUnique<T: Allele = DefaultAllele> {
    pub genes_size: usize,
    pub allele_list_sizes: Vec<usize>,
    pub allele_list_index_offsets: Vec<usize>,
    pub allele_lists: Vec<Vec<T>>,
    allele_list_index_sampler: WeightedIndex<usize>,
    allele_list_index_samplers: Vec<Uniform<usize>>,
    pub crossover_points: Vec<usize>,
    crossover_point_index_sampler: Option<Uniform<usize>>,
    pub seed_genes_list: Vec<Vec<T>>,
    pub chromosome_bin: Vec<MultiUniqueChromosome<T>>,
    pub best_genes: Vec<T>,
}

impl<T: Allele> TryFrom<Builder<Self>> for MultiUnique<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.allele_lists.is_none() {
            Err(TryFromBuilderError(
                "MultiUniqueGenotype requires a allele_lists",
            ))
        } else if builder.allele_lists.as_ref().map(|o| o.is_empty()).unwrap() {
            Err(TryFromBuilderError(
                "MultiUniqueGenotype requires non-empty allele_lists",
            ))
        } else {
            let allele_lists = builder.allele_lists.unwrap();
            let allele_list_sizes: Vec<usize> = allele_lists.iter().map(|v| v.len()).collect();
            // has one last index too many, but robust for chromosome_permutations_into_iter logic
            let allele_list_index_offsets =
                allele_list_sizes.iter().fold(vec![0], |mut acc, size| {
                    acc.push(*acc.last().unwrap() + size);
                    acc
                });

            let mut crossover_points = allele_list_index_offsets.clone();
            crossover_points.remove(0);
            crossover_points.pop();
            let crossover_point_index_sampler = if crossover_points.is_empty() {
                None
            } else {
                Some(Uniform::from(0..crossover_points.len()))
            };
            let genes_size = allele_list_sizes.iter().sum();

            Ok(Self {
                genes_size,
                allele_list_sizes: allele_list_sizes.clone(),
                allele_list_index_offsets: allele_list_index_offsets.clone(),
                allele_lists: allele_lists.clone(),
                allele_list_index_sampler: WeightedIndex::new(allele_list_sizes.clone()).unwrap(),
                allele_list_index_samplers: allele_list_sizes
                    .iter()
                    .map(|allele_value_size| Uniform::from(0..*allele_value_size))
                    .collect(),
                crossover_points,
                crossover_point_index_sampler,
                seed_genes_list: builder.seed_genes_list,
                chromosome_bin: vec![],
                best_genes: allele_lists.clone().into_iter().flatten().collect(),
            })
        }
    }
}

impl<T: Allele> Genotype for MultiUnique<T> {
    type Allele = T;
    type Genes = Vec<Self::Allele>;
    type Chromosome = MultiUniqueChromosome<Self::Allele>;

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
    fn best_genes_slice(&self) -> &[Self::Allele] {
        self.best_genes.as_slice()
    }
    fn genes_slice<'a>(&'a self, chromosome: &'a Self::Chromosome) -> &'a [Self::Allele] {
        chromosome.genes.as_slice()
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
            for _ in 0..number_of_mutations {
                let allele_list_index = self.allele_list_index_sampler.sample(rng);
                let allele_list_index_offset = self.allele_list_index_offsets[allele_list_index];
                let index1 = allele_list_index_offset
                    + self.allele_list_index_samplers[allele_list_index].sample(rng);
                let index2 = allele_list_index_offset
                    + self.allele_list_index_samplers[allele_list_index].sample(rng);
                chromosome.genes.swap(index1, index2);
            }
        } else {
            rng.sample_iter(&self.allele_list_index_sampler)
                .take(number_of_mutations)
                .fold(HashMap::<usize, usize>::new(), |mut m, x| {
                    *m.entry(x).or_default() += 1;
                    m
                })
                .into_iter()
                .for_each(|(allele_list_index, count)| {
                    let allele_list_size = self.allele_list_sizes[allele_list_index];
                    let allele_list_index_offset =
                        self.allele_list_index_offsets[allele_list_index];
                    rand::seq::index::sample(rng, allele_list_size, allele_list_size.min(count * 2))
                        .iter()
                        .tuples()
                        .for_each(|(index1, index2)| {
                            chromosome.genes.swap(
                                allele_list_index_offset + index1,
                                allele_list_index_offset + index2,
                            )
                        })
                });
        }
        chromosome.taint();
    }

    fn crossover_chromosome_genes<R: Rng>(
        &mut self,
        _number_of_crossovers: usize,
        _allow_duplicates: bool,
        _father: &mut Self::Chromosome,
        _mother: &mut Self::Chromosome,
        _rng: &mut R,
    ) {
        panic!("MultiUniqueGenotype does not support gene crossover")
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
            rng.sample_iter(self.crossover_point_index_sampler.unwrap())
                .take(number_of_crossovers)
                .for_each(|point_index| {
                    let gene_index = self.crossover_points[point_index];
                    let mother_back = &mut mother.genes[gene_index..];
                    let father_back = &mut father.genes[gene_index..];
                    father_back.swap_with_slice(mother_back);
                });
        } else {
            rand::seq::index::sample(
                rng,
                self.crossover_points.len(),
                number_of_crossovers.min(self.crossover_points.len()),
            )
            .iter()
            .sorted_unstable()
            .chunks(2)
            .into_iter()
            .for_each(|mut chunk| match (chunk.next(), chunk.next()) {
                (Some(start_point_index), Some(end_point_index)) => {
                    let start_gene_index = self.crossover_points[start_point_index];
                    let end_gene_index = self.crossover_points[end_point_index];
                    let mother_back = &mut mother.genes[start_gene_index..end_gene_index];
                    let father_back = &mut father.genes[start_gene_index..end_gene_index];
                    father_back.swap_with_slice(mother_back);
                }
                (Some(start_point_index), _) => {
                    let start_gene_index = self.crossover_points[start_point_index];
                    let mother_back = &mut mother.genes[start_gene_index..];
                    let father_back = &mut father.genes[start_gene_index..];
                    father_back.swap_with_slice(mother_back);
                }
                _ => (),
            });
        }
        mother.taint();
        father.taint();
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

impl<T: Allele> EvolveGenotype for MultiUnique<T> {}
impl<T: Allele> IncrementalGenotype for MultiUnique<T> {
    fn fill_neighbouring_population<R: Rng>(
        &mut self,
        chromosome: &Self::Chromosome,
        population: &mut Population<Self::Chromosome>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) {
        self.allele_list_sizes
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(index, allele_value_size)| {
                let index_offset: usize = self.allele_list_index_offsets[index];

                (0..allele_value_size)
                    .tuple_combinations()
                    .for_each(|(first, second)| {
                        let mut new_chromosome = self.chromosome_constructor_from(chromosome);
                        new_chromosome
                            .genes
                            .swap(index_offset + first, index_offset + second);
                        population.chromosomes.push(new_chromosome);
                    });
            });
    }

    fn neighbouring_population_size(&self) -> BigUint {
        self.allele_list_sizes
            .iter()
            .filter(|allele_value_size| **allele_value_size > 1)
            .map(|allele_value_size| {
                let n = BigUint::from(*allele_value_size);
                let k = BigUint::from(2usize);

                n.factorial() / (k.factorial() * (n - k).factorial())
            })
            .sum()
    }
}

impl<T: Allele> PermutableGenotype for MultiUnique<T> {
    fn chromosome_permutations_into_iter(
        &self,
    ) -> impl Iterator<Item = MultiUniqueChromosome<T>> + Send {
        self.allele_lists
            .clone()
            .into_iter()
            .map(|allele_list| {
                let size = allele_list.len();
                allele_list.into_iter().permutations(size)
            })
            .multi_cartesian_product()
            .map(|gene_sets| MultiUniqueChromosome::new(gene_sets.into_iter().concat()))
    }

    fn chromosome_permutations_size(&self) -> BigUint {
        self.allele_list_sizes
            .iter()
            .map(|v| BigUint::from(*v))
            .fold(BigUint::from(1u8), |acc, allele_list_size| {
                acc * allele_list_size.factorial()
            })
    }
}

impl<T: Allele> ChromosomeManager<Self> for MultiUnique<T> {
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<T> {
        if self.seed_genes_list.is_empty() {
            self.allele_lists
                .iter()
                .flat_map(|allele_list| {
                    let mut genes = allele_list.clone();
                    genes.shuffle(rng);
                    genes
                })
                .collect()
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn set_random_genes<R: Rng>(&mut self, chromosome: &mut MultiUniqueChromosome<T>, rng: &mut R) {
        chromosome.genes.clone_from(&self.random_genes_factory(rng));
    }
    fn copy_genes(
        &mut self,
        source: &MultiUniqueChromosome<T>,
        target: &mut MultiUniqueChromosome<T>,
    ) {
        target.genes.clone_from(&source.genes);
    }
    fn chromosome_bin_push(&mut self, chromosome: MultiUniqueChromosome<T>) {
        self.chromosome_bin.push(chromosome);
    }
    fn chromosome_bin_find_or_create(&mut self) -> MultiUniqueChromosome<T> {
        self.chromosome_bin.pop().unwrap_or_else(|| {
            let genes = Vec::with_capacity(self.genes_size);
            MultiUniqueChromosome::new(genes)
        })
    }
}

impl<T: Allele> fmt::Display for MultiUnique<T> {
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
        writeln!(
            f,
            "  expected_number_of_sampled_index_duplicates: {}",
            self.expected_number_of_sampled_index_duplicates_report()
        )?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes_list.len())
    }
}
