//! A solution strategy for finding the best chromosome, when crossover is impossible or inefficient
mod builder;
pub mod prelude;

pub use self::builder::{
    Builder as HillClimbBuilder, TryFromBuilderError as TryFromHillClimbBuilderError,
};

use super::Strategy;
use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::IncrementalGenotype;
use num::BigUint;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::fmt;

pub type RandomChromosomeProbability = f64;

#[derive(Clone, Debug)]
pub enum HillClimbVariant {
    Stochastic,
    SteepestSingle,
    SteepestPermutation,
}

/// There are 2 variants:
/// * [HillClimbVariant::Stochastic]: does not examine all neighbors before deciding how to move.
///   Rather, it selects a neighbor at random, and decides (based on the amount of improvement in
///   that neighbor) whether to move to that neighbor or to examine another
/// * [HillClimbVariant::SteepestSingle]: all neighbours (with single mutation only) are compared and the closest to the solution is chosen
///     * If it is a better chromosome than the current best the next round uses this chromosome as a starting
///       point and the scale is reset
///     * If there not, the scale is reduced by a factor to zoom in on the local solution
/// * [HillClimbVariant::SteepestPermutation]: all neighbours (permutation of all neighbouring mutations) are compared and the closest to the solution is chosen
///     * If it is a better chromosome than the current best the next round uses this chromosome as a starting
///       point and the scale is reset
///     * If there not, the scale is reduced by a factor to zoom in on the local solution
///
/// The fitness is calculated each round.
/// * If the fitness is worse, the mutation is undone and the next round is started
/// * If the fitness is equal or better, the mutated chromosome is taken for the next round.
///   It is important to update the best chromosome on equal fitness for diversity reasons
///
/// To avoid a local optimum, the `random_chromosome_probability` can be provided.
/// It seems much more efficient to insert random chromosomes in a single [HillClimb] run, than to
/// `call_repeatedly` from the [HillClimbBuilder].
///
/// See [HillClimbBuilder] for initialization options.
///
/// Example:
/// ```
/// use genetic_algorithm::strategy::hill_climb::prelude::*;
/// use genetic_algorithm::fitness::placeholders::SumContinuousGenotype;
///
/// // the search space
/// let genotype = ContinuousGenotype::builder() // f32 alleles
///     .with_genes_size(16)                     // 16 genes
///     .with_allele_range(0.0..1.0)             // values betwee 0.0 and 1.0
///     .with_allele_neighbour_range(-0.1..0.1)  // neighbouring step size or 0.1 in both directions
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let mut rng = rand::thread_rng(); // unused randomness provider implementing Trait rand::Rng
/// let hill_climb = HillClimb::builder()
///     .with_genotype(genotype)
///     .with_variant(HillClimbVariant::Stochastic) // use a random neighbouring mutation variant
///     .with_fitness(SumContinuousGenotype(1e-5))  // sum the gene values of the chromosomes with precision 0.00001
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the lowest sum
///     .with_scaling((1.0, 0.8))                  // start with neighbouring mutation scale 1.0 and multiply by 0.8 to zoom in on solution when stale
///     .with_target_fitness_score(0)              // goal is 16 times <= 0.00001 in the best chromosome
///     .with_max_stale_generations(1000)          // stop searching if there is no improvement in fitness score for 1000 generations
///     .with_random_chromosome_probability(0.1)   // try a random chromosome with probability 0.1 to avoid local optimum
///     .call(&mut rng)
///     .unwrap();
///
/// // it's all about the best chromosome after all
/// let best_chromosome = hill_climb.best_chromosome().unwrap();
/// assert_eq!(best_chromosome.genes, vec![0.0; 16])
/// ```
pub struct HillClimb<G: IncrementalGenotype, F: Fitness<Genotype = G>> {
    genotype: G,
    fitness: F,
    variant: HillClimbVariant,

    fitness_ordering: FitnessOrdering,
    max_stale_generations: Option<usize>,
    target_fitness_score: Option<FitnessValue>,
    random_chromosome_probability: RandomChromosomeProbability,
    scaling: Option<(f32, f32)>,

    pub current_iteration: usize,
    current_generation: usize,
    current_scaling: Option<f32>,
    best_chromosome: Option<Chromosome<G>>,
    pub best_generation: usize,
    pub neighbours_size: BigUint,
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>> Strategy<G> for HillClimb<G, F> {
    fn call<R: Rng>(&mut self, rng: &mut R) {
        self.current_generation = 0;
        self.reset_scaling();
        self.best_generation = 0;
        self.best_chromosome = Some(self.genotype.chromosome_factory(rng));
        let random_chromosome_sampler = Bernoulli::new(self.random_chromosome_probability).unwrap();

        while !self.is_finished() {
            if random_chromosome_sampler.sample(rng) {
                let working_chromosome = &mut self.genotype.chromosome_factory(rng);
                self.fitness.call_for_chromosome(working_chromosome);
                self.update_best_chromosome(working_chromosome);
            } else {
                match self.variant {
                    HillClimbVariant::Stochastic => {
                        let working_chromosome = &mut self.best_chromosome().unwrap();
                        self.genotype
                            .mutate_chromosome_neighbour(working_chromosome, rng);
                        self.fitness.call_for_chromosome(working_chromosome);
                        self.update_best_chromosome(working_chromosome);
                    }
                    HillClimbVariant::SteepestSingle => {
                        let working_chromosome = &mut self.best_chromosome().unwrap();
                        let mut working_chromosomes: Vec<Chromosome<G>> = self
                            .genotype
                            .chromosome_neighbours(working_chromosome, self.current_scaling);
                        working_chromosomes
                            .iter_mut()
                            .for_each(|chromosome| self.fitness.call_for_chromosome(chromosome));

                        let best_working_chromosome = match self.fitness_ordering {
                            FitnessOrdering::Maximize => working_chromosomes.iter().max(),
                            FitnessOrdering::Minimize => working_chromosomes
                                .iter()
                                .filter(|c| c.fitness_score.is_some())
                                .min(),
                        };
                        self.update_best_chromosome(
                            best_working_chromosome.unwrap_or(working_chromosome),
                        );
                    }
                    HillClimbVariant::SteepestPermutation => {
                        let working_chromosome = &mut self.best_chromosome().unwrap();
                        let mut working_chromosomes: Vec<Chromosome<G>> =
                            self.genotype.chromosome_neighbour_permutations(
                                working_chromosome,
                                self.current_scaling,
                            );
                        working_chromosomes
                            .iter_mut()
                            .for_each(|chromosome| self.fitness.call_for_chromosome(chromosome));

                        let best_working_chromosome = match self.fitness_ordering {
                            FitnessOrdering::Maximize => working_chromosomes.iter().max(),
                            FitnessOrdering::Minimize => working_chromosomes
                                .iter()
                                .filter(|c| c.fitness_score.is_some())
                                .min(),
                        };
                        self.update_best_chromosome(
                            best_working_chromosome.unwrap_or(working_chromosome),
                        );
                    }
                }
            }

            //self.report_round();
            self.current_generation += 1;
        }
    }
    fn best_chromosome(&self) -> Option<Chromosome<G>> {
        self.best_chromosome.clone()
    }
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>> HillClimb<G, F> {
    pub fn builder() -> HillClimbBuilder<G, F> {
        HillClimbBuilder::new()
    }

    fn update_best_chromosome(&mut self, contending_best_chromosome: &Chromosome<G>) {
        match self.best_chromosome.as_ref() {
            None => {
                self.best_chromosome = Some(contending_best_chromosome.clone());
            }
            Some(current_best_chromosome) => {
                match (
                    current_best_chromosome.fitness_score,
                    contending_best_chromosome.fitness_score,
                ) {
                    (None, None) => {}
                    (Some(_), None) => {}
                    (None, Some(_)) => {
                        self.best_chromosome = Some(contending_best_chromosome.clone());
                        self.best_generation = self.current_generation;
                    }
                    (Some(current_fitness_score), Some(contending_fitness_score)) => {
                        match self.fitness_ordering {
                            FitnessOrdering::Maximize => {
                                if contending_fitness_score >= current_fitness_score {
                                    self.best_chromosome = Some(contending_best_chromosome.clone());
                                    if contending_fitness_score > current_fitness_score {
                                        self.best_generation = self.current_generation;
                                        self.reset_scaling();
                                    } else {
                                        self.scale_down();
                                    }
                                }
                            }
                            FitnessOrdering::Minimize => {
                                if contending_fitness_score <= current_fitness_score {
                                    self.best_chromosome = Some(contending_best_chromosome.clone());
                                    if contending_fitness_score < current_fitness_score {
                                        self.best_generation = self.current_generation;
                                        self.reset_scaling();
                                    } else {
                                        self.scale_down();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.is_finished_by_max_stale_generations() || self.is_finished_by_target_fitness_score()
    }

    fn is_finished_by_max_stale_generations(&self) -> bool {
        if let Some(max_stale_generations) = self.max_stale_generations {
            self.current_generation - self.best_generation >= max_stale_generations
        } else {
            false
        }
    }

    fn is_finished_by_target_fitness_score(&self) -> bool {
        if let Some(target_fitness_score) = self.target_fitness_score {
            if let Some(fitness_score) = self.best_fitness_score() {
                match self.fitness_ordering {
                    FitnessOrdering::Maximize => fitness_score >= target_fitness_score,
                    FitnessOrdering::Minimize => fitness_score <= target_fitness_score,
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn report_round(&self) {
        println!(
            "current generation: {}, best fitness score: {:?}, current fitness score: {:?}, genes: {:?}",
            self.current_generation,
            self.best_fitness_score(),
            self.best_chromosome.as_ref().map(|o| &o.fitness_score),
            self.best_chromosome.as_ref().map(|o| &o.genes),
        );
    }

    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }

    fn reset_scaling(&mut self) {
        self.current_scaling = self.scaling.map(|(base, _factor)| base);
    }

    fn scale_down(&mut self) {
        if let Some(current_scaling) = self.current_scaling {
            self.current_scaling = Some(current_scaling * self.scaling.as_ref().unwrap().1);
        }
    }
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>> TryFrom<HillClimbBuilder<G, F>>
    for HillClimb<G, F>
{
    type Error = TryFromHillClimbBuilderError;

    fn try_from(builder: HillClimbBuilder<G, F>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires a Genotype",
            ))
        } else if builder.fitness.is_none() {
            Err(TryFromHillClimbBuilderError("HillClimb requires a Fitness"))
        } else if builder.max_stale_generations.is_none() && builder.target_fitness_score.is_none()
        {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires at least a max_stale_generations or target_fitness_score ending condition",
            ))
        } else {
            let genotype = builder.genotype.unwrap();
            let neighbours_size = genotype.chromosome_neighbours_size();

            Ok(Self {
                genotype: genotype,
                fitness: builder.fitness.unwrap(),
                variant: builder.variant.unwrap_or(HillClimbVariant::Stochastic),

                fitness_ordering: builder.fitness_ordering,
                max_stale_generations: builder.max_stale_generations,
                target_fitness_score: builder.target_fitness_score,
                random_chromosome_probability: builder.random_chromosome_probability.unwrap_or(0.0),
                scaling: builder.scaling,

                current_iteration: 0,
                current_generation: 0,
                current_scaling: None,
                best_generation: 0,
                best_chromosome: None,
                neighbours_size: neighbours_size,
            })
        }
    }
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>> fmt::Display for HillClimb<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hill_climb:")?;
        writeln!(f, "  genotype: {:?}", self.genotype)?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;

        writeln!(
            f,
            "  max_stale_generations: {:?}",
            self.max_stale_generations
        )?;
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  neighbours_size: {}", self.neighbours_size)?;

        writeln!(f, "  current iteration: {:?}", self.current_iteration)?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
