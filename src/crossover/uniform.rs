use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Crossover with 50% probability for each gene to come from one of the two parents.
/// Optionally keep parents around to compete with children later on.
///
/// Not allowed for unique genotypes as it would not preserve the gene uniqueness in the children.
#[derive(Clone, Debug)]
pub struct Uniform {
    pub keep_parent: bool,
}
impl Crossover for Uniform {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Allele = G::Allele>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G::Allele>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        if state.population.size() < 2 {
            return;
        }
        let crossover_indexes = genotype.crossover_indexes();
        let bool_sampler = Bernoulli::new(0.5).unwrap();
        let mut parent_chromosomes = if self.keep_parent {
            state.population.chromosomes.clone()
        } else {
            vec![] // throwaway to keep compiler happy
        };

        state
            .population
            .chromosomes
            .chunks_mut(2)
            .for_each(|chunk| {
                if let [father, mother] = chunk {
                    for index in &crossover_indexes {
                        if bool_sampler.sample(rng) {
                            std::mem::swap(&mut father.genes[*index], &mut mother.genes[*index]);
                        }
                    }
                    mother.taint_fitness_score();
                    father.taint_fitness_score();
                }
            });

        if self.keep_parent {
            state.population.chromosomes.append(&mut parent_chromosomes);
        }
    }
    fn require_crossover_indexes(&self) -> bool {
        true
    }
    fn require_crossover_points(&self) -> bool {
        false
    }
}

impl Uniform {
    pub fn new(keep_parent: bool) -> Self {
        Self { keep_parent }
    }
}
